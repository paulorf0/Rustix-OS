#[unsafe(naked)]
extern "sysv64" fn common_stub() {
    core::arch::naked_asm!(
        // 1. Save general-purpose registers
        "push rax", "push rcx", "push rdx", "push rbx",
        "push rbp", "push rsi", "push rdi",
        "push r8",  "push r9",  "push r10", "push r11",
        "push r12", "push r13", "push r14", "push r15",

        // 2. Pass the whole stack pointer as 1st argument (RDI)
        "mov rdi, rsp",

        // 3. Call the same central Rust function
        "call {handler}",
        //
        // 4. Restore registers and clean up the pushed error/vector
        "pop r15", "pop r14", "pop r13", "pop r12",
        "pop r11", "pop r10", "pop r9",  "pop r8",
        "pop rdi", "pop rsi", "pop rbp",
        "pop rbx", "pop rdx", "pop rcx", "pop rax",
        "add rsp, 16", // Remove 'vector' and 'error_code' from the stack

        "iretq",
        handler = sym interrupt_dispatch,
    );
}

// Preserves the order of the fields.
#[repr(C)]
struct InterruptFrame {
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,

    pub vector: u64,
    pub error_code: u64,

    pub rip: u64,
    pub cs: u64,
    pub rflags: u64,
    pub rsp: u64,
    pub ss: u64,
}

// SysV64 ABI: RDI carries the 1st arg. Stack top (RSP) was moved into RDI
// just before this call, so interrupt_dispatch receives it as its first argument.
extern "sysv64" fn interrupt_dispatch(frame: *const InterruptFrame) {
    let frame = unsafe { &*frame };

    // match frame.vector {
    //     _ => todo!(),
    // }
    todo!();
}
