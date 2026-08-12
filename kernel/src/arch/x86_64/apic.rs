use crate::isolation::scheduler::SCHEDULER;
use core::arch::naked_asm;
use core::ptr::addr_of_mut;

const APIC_BASE_MSR: u32 = 0x1B;
const APIC_LVT_TIMER: u32 = 0x320;
const APIC_TIMER_INIT_CNT: u32 = 0x380;
const APIC_TIMER_DIV_CONFIG: u32 = 0x3E0;
const APIC_EOI: u32 = 0x0B0;

#[allow(dead_code)]
pub fn init_apic_timer() {
    unsafe {
        let apic_msr = rdmsr(APIC_BASE_MSR);
        let apic_base = apic_msr & 0xFFFF_F000;

        if apic_base == 0 {
            return;
        }

        let apic_mem = apic_base as *mut u32;

        let lvt_timer_ptr = apic_mem.add(APIC_LVT_TIMER as usize / 4);
        lvt_timer_ptr.write_volatile(32 | 0x20000); // Vector 32, Periodic Mode

        let div_config_ptr = apic_mem.add(APIC_TIMER_DIV_CONFIG as usize / 4);
        div_config_ptr.write_volatile(0x3);

        let init_cnt_ptr = apic_mem.add(APIC_TIMER_INIT_CNT as usize / 4);
        init_cnt_ptr.write_volatile(100_000);
    }
}

#[inline]
unsafe fn rdmsr(msr: u32) -> u64 {
    let low: u32;
    let high: u32;
    core::arch::asm!(
        "rdmsr",
        in("ecx") msr,
        out("eax") low,
        out("edx") high,
        options(nomem, nostack, preserves_flags)
    );
    ((high as u64) << 32) | (low as u64)
}

#[allow(dead_code)]
#[unsafe(naked)]
pub unsafe extern "C" fn apic_timer_stub() {
    naked_asm!(
        "cli",
        "push rax",
        "push rcx",
        "push rdx",
        "push rbx",
        "push rbp",
        "push rsi",
        "push rdi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "call apic_timer_handler",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rbp",
        "pop rbx",
        "pop rdx",
        "pop rcx",
        "pop rax",
        "iretq"
    );
}

#[no_mangle]
pub extern "C" fn apic_timer_handler() {
    unsafe {
        // Trigger Preemptive Task Switching on Timer Tick
        let sched = &mut *addr_of_mut!(SCHEDULER);
        sched.schedule_next();

        // Send EOI (End of Interrupt)
        let apic_msr = rdmsr(APIC_BASE_MSR);
        let apic_base = apic_msr & 0xFFFF_F000;
        if apic_base != 0 {
            let eoi_ptr = (apic_base as *mut u32).add(APIC_EOI as usize / 4);
            eoi_ptr.write_volatile(0);
        }
    }
}
