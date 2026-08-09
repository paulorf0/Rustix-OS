// TODO: Interrupt handlers where the CPU pushes an error code onto the stack (exceptions with error code).
// #DF(8) and #AC(17) always push 0. The others (#TS 10, #NP 11, #SS 12, #GP 13, #PF 14, #CP 21)
// push the actual error code (selector/page index, etc.).
// Others without an error code automatically pushed by the CPU: #DE(0), #DB(1), NMI(2), #BP(3), #OF(4),
// #BR(5), #UD(6), #NM(7), #MF(16), #XM(19), #VE(20).

use crate::kernel_interrupt_service::isr::InterruptFrame;
use core::fmt::Write;
use uart_16550::Uart16550Tty;

pub fn generic_handler(frame: &InterruptFrame) -> ! {
    let mut serial = unsafe {
        Uart16550Tty::new_port(0x3f8, uart_16550::Config::default())
            .expect("should initialize device")
    };

    let _ = write!(
        serial,
        "\n[EXCEPTION]\nVector: {}\nError Code: {}\nRIP: {:#x}\nCS: {:#x}\nRFLAGS: {:#x}\nRSP: {:#x}\nSS: {:#x}",
        frame.vector, frame.error_code, frame.rip, frame.cs, frame.rflags, frame.rsp, frame.ss
    );

    loop {
        unsafe { core::arch::asm!("hlt") }
    }
}
