#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;

use uefi::fs::FileSystem;
use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, MemoryType};
use uefi::CString16;

mod elf_loader;

#[entry]
fn efi_main(image: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if uefi::helpers::init(&mut system_table).is_err() {
        return Status::ABORTED;
    }

    let loaded = {
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

        if kernel_elf.is_empty() {
            return Status::LOAD_ERROR;
        }

        let memory_map = match boot_services.memory_map(MemoryType::CONVENTIONAL) {
            Ok(map) => map,
            Err(_) => return Status::ABORTED,
        };

        let descriptors: Vec<MemoryDescriptor> = memory_map.entries().copied().collect();

        unsafe {
            match elf_loader::parse_and_load_kernel(&kernel_elf, &descriptors, boot_services) {
                Ok(info) => info,
                Err(_) => return Status::LOAD_ERROR,
            }
        }
    };

    let (_runtime_system_table, _final_memory_map) =
        unsafe { system_table.exit_boot_services(MemoryType::LOADER_DATA) };

    // ------------------------------------------------------------
    // 5. Jump to the loaded kernel
    // ------------------------------------------------------------

    let entry = loaded.entry_point_virt;

    let kernel_entry: extern "C" fn() -> ! = unsafe { core::mem::transmute(entry) };

    kernel_entry();
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
