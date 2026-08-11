#![no_main]
#![no_std]

mod gdt;
mod kernel_interrupt_service;

use bootloader_api::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use uart_16550::Uart16550Tty;

#[unsafe(naked)]
extern "sysv64" fn pic8259_remap() {
    core::arch::naked_asm!(
        // ICW1: init + ICW4 needed
        "mov al, 0x11",
        "out 0x20, al",
        "out 0x80, al", // io_wait
        "out 0xA0, al",
        "out 0x80, al",
        // ICW2: base vectors
        "mov al, 32",
        "out 0x21, al",
        "out 0x80, al",
        "mov al, 40",
        "out 0xA1, al",
        "out 0x80, al",
        // ICW3: cascade wiring
        "mov al, 0x04",
        "out 0x21, al",
        "out 0x80, al",
        "mov al, 0x02",
        "out 0xA1, al",
        "out 0x80, al",
        // ICW4: 8086 mode
        "mov al, 0x01",
        "out 0x21, al",
        "out 0x80, al",
        "out 0xA1, al",
        "out 0x80, al",
        // OCW1: mask all IRQs until handlers exist
        "mov al, 0xFF",
        "out 0x21, al",
        "out 0xA1, al",
        "ret",
    );
}

entry_point!(kernel_main);
fn kernel_main(_boot_info: &mut BootInfo) -> ! {
    gdt::setup_gdt();
    pic8259_remap();
    kernel_interrupt_service::idt::idt_init();

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut serial = unsafe {
        Uart16550Tty::new_port(0x3f8, uart_16550::Config::default())
            .expect("should initialize device")
    };

    let location = match info.location() {
        Some(l) => l,
        None => todo!(),
    };

    let msg = match info.message().as_str() {
        Some(m) => m,
        None => todo!(),
    };

    let _ = write!(
        serial,
        "\n[ERROR]\nFile Name: {}\nLine: {}\nColumn: {}\nError Message: {}",
        location.file(),
        location.line(),
        location.column(),
        msg
    );

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
