use core::arch::asm;

/// Enters User Space (Ring 3) using IRETQ stack frame.
/// Safety: `user_code_entry` and `user_stack` must be valid virtual addresses mapped with USER_ACCESSIBLE flags.
pub unsafe fn jump_to_user_mode(user_code_entry: u64, user_stack: u64) -> ! {
    let user_cs: u64 = 0x1B; // User Code Selector (GDT Offset 0x18 | RPL 3)
    let user_ss: u64 = 0x23; // User Data Selector (GDT Offset 0x20 | RPL 3)
    let rflags: u64 = 0x202; // IF (Interrupt Flag) enabled + Reserved bit set

    asm!(
        "cli",
        "push {0}", // User SS
        "push {1}", // User RSP
        "push {2}", // RFLAGS
        "push {3}", // User CS
        "push {4}", // User RIP (Entry Point)
        "iretq",
        in(reg) user_ss,
        in(reg) user_stack,
        in(reg) rflags,
        in(reg) user_cs,
        in(reg) user_code_entry,
        options(noreturn)
    );
}

/// Dummy User Code for testing Ring 3 execution & SYSCALL
#[no_mangle]
#[unsafe(link_section = ".text")]
pub extern "C" fn test_user_code() -> ! {
    // Execute a SYSCALL to test Ring 3 -> Ring 0 Transition
    unsafe {
        asm!(
            "mov rax, 1",    // Syscall Number 1
            "mov rdi, 0x41", // Arg1: 'A'
            "syscall",
            options(noreturn)
        );
    }
}
