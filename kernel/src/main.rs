#![no_std]
#![no_main]

use core::panic::PanicInfo;

const STACK_SIZE: usize = 4096 * 8;

#[allow(dead_code)]
pub struct KernelStack([u8; STACK_SIZE]);

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
