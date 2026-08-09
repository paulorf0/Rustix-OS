/*
* Example type_attributes values (DPL = 0)
* 64-bit Interrupt Gate: 0x8E (p=1, dpl=0b00, type=0b1110 => type_attributes=0b1000_1110=0x8E)
*/

#[repr(C, packed)]
#[derive(Clone, Copy)]
struct interrupt_descriptor {
    offset_1: u16,
    selector: u16, // Code segment selector in gdt.
    ist: u8,
    type_attributes: u8, // Gate Type, DPL and P fields.
    offset_2: u16,
    offset_3: u32,
    zero: u32,
}

#[repr(C, packed)]
struct idtr {
    limit: u16,
    base: u64,
}

static mut IDT: [interrupt_descriptor; 256] = [interrupt_descriptor {
    offset_1: 0,
    selector: 0,
    ist: 0,
    type_attributes: 0,
    offset_2: 0,
    offset_3: 0,
    zero: 0,
}; 256];
static mut IDTR: idtr = idtr { limit: 0, base: 0 };

fn idt_set_gate(index: usize, isr: *const (), attributes: u8) {
    let addr = isr as u64;
    unsafe {
        IDT[index].offset_1 = (addr & 0xFFFF) as u16;
        IDT[index].selector = 0x0008; // 0b000000001000_0_00 = Offset - TI - RPL = 8 - GDT - KERNEL
        IDT[index].ist = 0;
        IDT[index].type_attributes = attributes;
        IDT[index].offset_2 = ((addr >> 16) & 0xFFFF) as u16;
        IDT[index].offset_3 = ((addr >> 32) & 0xFFFFFFFF) as u32;
        IDT[index].zero = 0;
    }
}

fn idt_load() {
    let idtr_addr = core::ptr::addr_of!(IDTR);
    unsafe {
        core::arch::asm!(
            "lidt [rdi]",
            "sti",
            in("rdi") idtr_addr,
            options(nostack)
        );
    }
}

pub fn idt_init() {
    unsafe {
        IDTR.base = core::ptr::addr_of!(IDT) as u64;
        IDTR.limit = (core::mem::size_of::<interrupt_descriptor>() * 256 - 1) as u16;
    }

    // Set gates.
    idt_load();
}
