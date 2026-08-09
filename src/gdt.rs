#[repr(C, packed)]
struct gdtr {
    limit: u16,
    base: u64,
}

#[repr(C, packed)]
struct gdt_entry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

// The OS has 3 descriptor table entries, just for the kernel: the null descriptor, the kernel code descriptor, and the kernel data descriptor.
static mut GDTR: gdtr = gdtr { limit: 0, base: 0 };
static mut GDT: [gdt_entry; 3] = [
    gdt_entry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
    gdt_entry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
    gdt_entry {
        limit_low: 0,
        base_low: 0,
        base_middle: 0,
        access: 0,
        granularity: 0,
        base_high: 0,
    },
];

unsafe fn gdt_fill_entry(num: usize, access: u8, granularity: u8, base: u32, limit: u32) {
    let gdt = &mut GDT[num];

    gdt.limit_low = (limit & 0xFFFF) as u16;
    gdt.base_low = (base & 0xFFFF) as u16;
    gdt.base_middle = ((base >> 16) & 0xFF) as u8;
    gdt.access = access;
    gdt.granularity = (((limit >> 16) & 0x0F) as u8) | (granularity & 0xF0);
    gdt.base_high = ((base >> 24) & 0xFF) as u8;
}

pub fn setup_gdt() {
    // Access field
    // 0bP_DPL_DPL_S_E_DC_RW_A
    // Base/limit ignored by the CPU for code/data segments in long mode.
    // Granularity high nibble carries the L bit (bit 5, long-mode code segment) for the
    // kernel code descriptor; irrelevant for null/data descriptors at this stage.

    unsafe {
        gdt_fill_entry(0x0, 0x00, 0x00, 0x0, 0x0); // null descriptor
        gdt_fill_entry(0x1, 0x9A, 0x20, 0x0, 0x0); // kernel code descriptor (L bit set)
        gdt_fill_entry(0x2, 0x92, 0x00, 0x0, 0x0); // kernel data descriptor

        GDTR.limit = (core::mem::size_of::<[gdt_entry; 3]>() - 1) as u16;
        GDTR.base = core::ptr::addr_of!(GDT) as u64;

        let gdtptr_addr = core::ptr::addr_of!(GDTR);

        core::arch::asm!(
            "lgdt [{0}]",
            in(reg) gdtptr_addr,
        );
        core::arch::asm!(
            "mov ax, 0x10",
            "mov ds, ax",
            "mov ss, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "push 0x08",
            "lea rax, [rip + 2f]",
            "push rax",
            "retfq",
            "2:",
            out("rax") _,
        );
    }
}
