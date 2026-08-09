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

fn idt_set_descriptor(vector: u8, isr: *const (), attributes: u8) {
    let addr = isr as u64;
    unsafe {
        IDT[vector as usize].offset_1 = (addr & 0xFFFF) as u16;
        IDT[vector as usize].selector = 0x08;
        IDT[vector as usize].ist = 0;
        IDT[vector as usize].type_attributes = attributes;
        IDT[vector as usize].offset_2 = ((addr >> 16) & 0xFFFF) as u16;
        IDT[vector as usize].offset_3 = ((addr >> 32) & 0xFFFFFFFF) as u32;
        IDT[vector as usize].zero = 0;
    }
}

fn idt_init() {
    unsafe {
        IDTR.base = core::ptr::addr_of!(IDT) as u64;
        IDTR.limit = (core::mem::size_of::<interrupt_descriptor>() * 256 - 1) as u16;

        // Example: Register the handler for Vector 32 (commonly Timer)
        // 0x8E = Present, Ring 0, Interrupt Gate
        /*         idt_set_descriptor(32, isr_handler_stub as *const (), 0x8E); */

        let idtr_addr = core::ptr::addr_of!(IDTR);
        // Load IDT table into the CPU register

        // TODO: Create a function with this code.
        // TODO: Create isr_handler_stub.
        core::arch::asm!(
            "lidt [rdi]",
            "sti",
            in("rdi") idtr_addr, //warning in this line.
            options(nomem, nostack)
        );
    }
}
