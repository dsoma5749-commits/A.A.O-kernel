#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use uefi::prelude::*;
use uefi::fs::FileSystem;
use uefi::CString16;

mod elf_loader;

#[entry]
fn efi_main(
    image: Handle,
    mut system_table: SystemTable<Boot>,
) -> Status {
    // Initialize UEFI helpers and the global allocator.
    if uefi::helpers::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    // uefi 0.29 uses the BootServices API through SystemTable.
    let boot_services = system_table.boot_services();

    let fs_protocol = match boot_services.get_image_file_system(image) {
        Ok(protocol) => protocol,
        Err(_) => return Status::NOT_FOUND,
    };

    let mut fs = FileSystem::new(fs_protocol);

    let kernel_path = match CString16::try_from("\\kernel.elf") {
        Ok(path) => path,
        Err(_) => return Status::INVALID_PARAMETER,
    };

    let kernel_elf: Vec<u8> = match fs.read(kernel_path.as_ref()) {
        Ok(data) => data,
        Err(_) => return Status::NOT_FOUND,
    };

    // Checkpoint: kernel.elf was successfully read.
    let _kernel_size = kernel_elf.len();

    Status::SUCCESS
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
