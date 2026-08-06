#![no_main]
#![no_std]

use bootloader_api::{BootInfo, entry_point};
use core::{fmt::Write, panic::PanicInfo};
use uart_16550::Uart16550Tty;

entry_point!(kernel_main);
fn kernel_main(boot_info: &mut BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().expect("No framebuffer.");
    let info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();
    let bytes_per_pixel = info.bytes_per_pixel;

    let bytes_middle_hscreen = (info.height / 2) * info.stride * bytes_per_pixel;

    for x_coordinate in 0..info.width {
        let pixel_coordinate = bytes_middle_hscreen + x_coordinate * bytes_per_pixel;

        buffer[pixel_coordinate] = 0;
        buffer[pixel_coordinate + 1] = 255;
        buffer[pixel_coordinate + 2] = 0;
    }

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

    let _ = write!(serial, "[TEST] SHOWING?");
    // (...).write_str dont accept the args, just &str.
    // let _ = serial.write_str(
    // "[ERROR]\nFile Name: {}\nLine: {}\nColumn: {}",
    // location.file(),
    // location.line(),
    // location.column(),
    // );

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
