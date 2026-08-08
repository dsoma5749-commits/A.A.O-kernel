#![no_std]
#![no_main]

extern crate alloc;

use uefi::prelude::*;

mod elf_loader;

#[entry]
fn efi_main(
    _image: Handle,
    mut system_table: SystemTable<Boot>,
) -> Status {
    if uefi::helpers::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
