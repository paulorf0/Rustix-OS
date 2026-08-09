use crate::kernel_interrupt_service::handlers;

// Preserves the order of the fields.
#[repr(C)]
pub struct InterruptFrame {
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

// Without error code.
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec0() {
    core::arch::naked_asm!(
        "push 0",
        "push 0",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec1() {
    core::arch::naked_asm!(
        "push 0",
        "push 1",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec2() {
    core::arch::naked_asm!(
        "push 0",
        "push 2",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec3() {
    core::arch::naked_asm!(
        "push 0",
        "push 3",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec4() {
    core::arch::naked_asm!(
        "push 0",
        "push 4",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec5() {
    core::arch::naked_asm!(
        "push 0",
        "push 5",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec6() {
    core::arch::naked_asm!(
        "push 0",
        "push 6",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec7() {
    core::arch::naked_asm!(
        "push 0",
        "push 7",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec9() {
    core::arch::naked_asm!(
        "push 0",
        "push 9",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec16() {
    core::arch::naked_asm!(
        "push 0",
        "push 16",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec18() {
    core::arch::naked_asm!(
        "push 0",
        "push 18",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec19() {
    core::arch::naked_asm!(
        "push 0",
        "push 19",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec20() {
    core::arch::naked_asm!(
        "push 0",
        "push 20",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec15() {
    core::arch::naked_asm!(
        "push 0",
        "push 15",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec22() {
    core::arch::naked_asm!(
        "push 0",
        "push 22",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec23() {
    core::arch::naked_asm!(
        "push 0",
        "push 23",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec24() {
    core::arch::naked_asm!(
        "push 0",
        "push 24",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec25() {
    core::arch::naked_asm!(
        "push 0",
        "push 25",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec26() {
    core::arch::naked_asm!(
        "push 0",
        "push 26",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec27() {
    core::arch::naked_asm!(
        "push 0",
        "push 27",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec28() {
    core::arch::naked_asm!(
        "push 0",
        "push 28",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

#[unsafe(naked)]
pub extern "sysv64" fn stub_vec31() {
    core::arch::naked_asm!(
        "push 0",
        "push 31",
        "jmp {common_stub}",
        common_stub = sym common_stub,
    )
}

// With error code added by cpu.
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec8() {
    core::arch::naked_asm!("push 8", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec10() {
    core::arch::naked_asm!("push 10", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec11() {
    core::arch::naked_asm!("push 11", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec12() {
    core::arch::naked_asm!("push 12", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec13() {
    core::arch::naked_asm!("push 13", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec14() {
    core::arch::naked_asm!("push 14", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec17() {
    core::arch::naked_asm!("push 17", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec21() {
    core::arch::naked_asm!("push 21", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec29() {
    core::arch::naked_asm!("push 29", "jmp {common_stub}", common_stub = sym common_stub)
}
#[unsafe(naked)]
pub extern "sysv64" fn stub_vec30() {
    core::arch::naked_asm!("push 30", "jmp {common_stub}", common_stub = sym common_stub)
}

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

// SysV64 ABI: RDI carries the 1st arg. Stack top (RSP) was moved into RDI
// just before this call, so interrupt_dispatch receives it as its first argument.
extern "sysv64" fn interrupt_dispatch(frame: *const InterruptFrame) -> ! {
    let frame = unsafe { &*frame };

    handlers::generic_handler(frame)
}
