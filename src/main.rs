#![no_main]
#![no_std]

mod gdt;
mod kernel_interrupt_service;

use bootloader_api::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use uart_16550::Uart16550Tty;

entry_point!(kernel_main);
fn kernel_main(_boot_info: &mut BootInfo) -> ! {
    gdt::setup_gdt();
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
