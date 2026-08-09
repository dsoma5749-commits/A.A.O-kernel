#![no_std]
#![no_main]

use core::panic::PanicInfo;

const STACK_SIZE: usize = 16 * 1024;

#[repr(align(16))]
#[allow(dead_code)]
pub struct KernelStack([u8; STACK_SIZE]);

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss")]
pub static mut KERNEL_STACK: KernelStack =
    KernelStack([0; STACK_SIZE]);

#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".bss")]
pub static mut AAO_BSS_TEST: [u8; 4096] = [0; 4096];

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
