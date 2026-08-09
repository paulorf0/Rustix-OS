/*
* Example type_attributes values (DPL = 0)
* 64-bit Interrupt Gate: 0x8E (p=1, dpl=0b00, type=0b1110 => type_attributes=0b1000_1110=0x8E)
*/

use crate::kernel_interrupt_service::isr;

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
    idt_set_gate(0, isr::stub_vec0 as *const (), 0x8E);
    idt_set_gate(1, isr::stub_vec1 as *const (), 0x8E);
    idt_set_gate(2, isr::stub_vec2 as *const (), 0x8E);
    idt_set_gate(3, isr::stub_vec3 as *const (), 0x8E);
    idt_set_gate(4, isr::stub_vec4 as *const (), 0x8E);
    idt_set_gate(5, isr::stub_vec5 as *const (), 0x8E);
    idt_set_gate(6, isr::stub_vec6 as *const (), 0x8E);
    idt_set_gate(7, isr::stub_vec7 as *const (), 0x8E);
    idt_set_gate(8, isr::stub_vec8 as *const (), 0x8E);
    idt_set_gate(9, isr::stub_vec9 as *const (), 0x8E);
    idt_set_gate(10, isr::stub_vec10 as *const (), 0x8E);
    idt_set_gate(11, isr::stub_vec11 as *const (), 0x8E);
    idt_set_gate(12, isr::stub_vec12 as *const (), 0x8E);
    idt_set_gate(13, isr::stub_vec13 as *const (), 0x8E);
    idt_set_gate(14, isr::stub_vec14 as *const (), 0x8E);
    idt_set_gate(15, isr::stub_vec15 as *const (), 0x8E);
    idt_set_gate(16, isr::stub_vec16 as *const (), 0x8E);
    idt_set_gate(17, isr::stub_vec17 as *const (), 0x8E);
    idt_set_gate(18, isr::stub_vec18 as *const (), 0x8E);
    idt_set_gate(19, isr::stub_vec19 as *const (), 0x8E);
    idt_set_gate(20, isr::stub_vec20 as *const (), 0x8E);
    idt_set_gate(21, isr::stub_vec21 as *const (), 0x8E);
    idt_set_gate(22, isr::stub_vec22 as *const (), 0x8E);
    idt_set_gate(23, isr::stub_vec23 as *const (), 0x8E);
    idt_set_gate(24, isr::stub_vec24 as *const (), 0x8E);
    idt_set_gate(25, isr::stub_vec25 as *const (), 0x8E);
    idt_set_gate(26, isr::stub_vec26 as *const (), 0x8E);
    idt_set_gate(27, isr::stub_vec27 as *const (), 0x8E);
    idt_set_gate(28, isr::stub_vec28 as *const (), 0x8E);
    idt_set_gate(29, isr::stub_vec29 as *const (), 0x8E);
    idt_set_gate(30, isr::stub_vec30 as *const (), 0x8E);
    idt_set_gate(31, isr::stub_vec31 as *const (), 0x8E);

    idt_load();
}
