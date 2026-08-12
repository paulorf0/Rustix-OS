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

// Stack alignment invariant at the `call` below:
//   5 qwords pushed by the CPU (SS, RSP, RFLAGS, CS, RIP)
// + 2 qwords pushed by the stub (error code, vector)
// + 15 qwords pushed here (general-purpose registers)
// = 22 qwords = 176 bytes, which is divisible by 16.
//
// The CPU aligns RSP to 16 before pushing the interrupt frame in long mode, so
// the starting point is guaranteed and RSP is still 16-byte aligned at the call.
// The System V ABI requires that, otherwise SSE spills such as `movaps` inside
// the handler fault with #GP. Adding or removing a push here breaks it: keep the
// total number of pushed qwords even.
#[unsafe(naked)]
extern "sysv64" fn common_stub() {
    core::arch::naked_asm!(
        // Save context
        "push rax", "push rcx", "push rdx", "push rbx",
        "push rbp", "push rsi", "push rdi",
        "push r8",  "push r9",  "push r10", "push r11",
        "push r12", "push r13", "push r14", "push r15",

        "mov rdi, rsp",

        // Call interrupt_dispatch
        "call {handler}",

        // Restore context
        "pop r15", "pop r14", "pop r13", "pop r12",
        "pop r11", "pop r10", "pop r9",  "pop r8",
        "pop rdi", "pop rsi", "pop rbp",
        "pop rbx", "pop rdx", "pop rcx", "pop rax",
        "add rsp, 16", // Remove 'vector' and 'error_code' from the stack

        // Return of interrupt function.
        "iretq",
        handler = sym interrupt_dispatch,
    );
}

const PIC_MASTER_CMD: u16 = 0x20;
const PIC_SLAVE_CMD: u16 = 0xA0;
const PIC_EOI: u8 = 0x20;

// Acknowledges the interrupt so the PIC clears its in-service bit and can deliver
// the next IRQ. Vectors 40 and above come from the slave PIC, which must be
// acknowledged before the master: the master still holds cascade line IRQ2 marked
// as in service, and releasing it first would open a window for a new IRQ while
// the slave is still busy.
unsafe fn send_eoi(vector: u64) {
    unsafe {
        if vector >= 40 {
            core::arch::asm!("out dx, al", in("dx") PIC_SLAVE_CMD, in("al") PIC_EOI);
        }
        core::arch::asm!("out dx, al", in("dx") PIC_MASTER_CMD, in("al") PIC_EOI);
    }
}

// SysV64 ABI: RDI carries the 1st arg. Stack top (RSP) was moved into RDI
// just before this call, so interrupt_dispatch receives it as its first argument.
//
// Returns so that `common_stub` can restore the context and run `iretq`. Vectors
// below 32 are CPU exceptions and divert to generic_handler, which never returns.
extern "sysv64" fn interrupt_dispatch(frame: *const InterruptFrame) {
    let frame = unsafe { &*frame };

    if frame.vector < 32 {
        handlers::generic_handler(frame);
    }

    // TODO: dispatch to a per-IRQ handler before acknowledging.

    unsafe { send_eoi(frame.vector) };
}
