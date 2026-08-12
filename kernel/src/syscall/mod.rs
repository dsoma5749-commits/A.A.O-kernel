use core::arch::{asm, naked_asm};

// MSR (Model Specific Registers) for Syscall configuration
const IA32_EFER: u32 = 0xC000_0080;
const IA32_STAR: u32 = 0xC000_0081;
const IA32_LSTAR: u32 = 0xC000_0082;
const IA32_FMASK: u32 = 0xC000_0084;

const EFER_SCE: u64 = 1 << 0; // System Call Extensions bit

#[inline]
unsafe fn wrmsr(msr: u32, value: u64) {
    let low = value as u32;
    let high = (value >> 32) as u32;
    asm!(
        "wrmsr",
        in("ecx") msr,
        in("eax") low,
        in("edx") high,
        options(nomem, nostack, preserves_flags)
    );
}

/// Initializes Fast Syscall registers on x86_64 CPU
pub fn init() {
    unsafe {
        // Enable SCE (System Call Extensions) in EFER MSR
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | EFER_SCE);

        // Setup CS/SS Selectors in STAR MSR
        // Kernel CS = 0x08, Kernel SS = 0x10, User CS = 0x1B, User SS = 0x23
        let star = ((0x08u64) << 32) | ((0x10u64 | 3) << 48);
        wrmsr(IA32_STAR, star);

        // Set Syscall Entry Point Address in LSTAR MSR
        wrmsr(IA32_LSTAR, syscall_entry as *const () as usize as u64);

        // Mask Interrupts on Syscall Entry in FMASK MSR (Mask IF bit 0x200)
        wrmsr(IA32_FMASK, 0x200);
    }
}

#[inline]
unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
        options(nomem, nostack, preserves_flags)
    );
    ((high as u64) << 32) | (low as u64)
}

/// Low-level naked assembly wrapper for SYSCALL instruction
#[unsafe(naked)]
pub unsafe extern "C" fn syscall_entry() {
    naked_asm!(
        // Save user-space stack & switch to kernel stack
        "cli",
        "mov r12, rsp",
        "mov rsp, offset KERNEL_STACK",
        // Pass arguments to Rust C-ABI Handler
        // RAX = Syscall Number, RDI = Arg1, RSI = Arg2, RDX = Arg3
        "mov rdi, rax",
        "mov rsi, rdi",
        "call syscall_handler",
        // Restore stack & Return to User-Space via SYSRET
        "mov rsp, r12",
        "sysretq"
    );
}

/// High-level Rust Syscall Dispatcher
#[no_mangle]
pub extern "C" fn syscall_handler(syscall_num: u64, arg1: u64) -> u64 {
    match syscall_num {
        1 => {
            // Syscall: Yield / Print Test
            0
        }
        2 => {
            // Syscall: Capability IPC Send
            arg1
        }
        _ => 0xFFFF_FFFF_FFFF_FFFF, // Invalid Syscall
    }
}
