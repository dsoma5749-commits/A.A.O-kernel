#![no_std]
#![no_main]

use core::fmt::Write;
use uefi::prelude::*;

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    let _boot_services = system_table.boot_services();

    let stdout = system_table.stdout();
    let _ = stdout.write_str("A.A.O Bootloader\r\n");
    let _ = stdout.write_str("UEFI 0.29 initialized\r\n");
    let _ = stdout.write_str("Boot environment OK\r\n");

    Status::SUCCESS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
