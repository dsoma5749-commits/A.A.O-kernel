#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;

use uefi::fs::FileSystem;
use uefi::prelude::*;
use uefi::table::boot::{AllocateType, MemoryDescriptor, MemoryType};
use uefi::CString16;

mod boot_info;
mod elf_loader;

use boot_info::BootInfo;

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

    /*
     * Deterministic Boot:
     * Currently using 0 for physical_memory_offset (Identity Mapped space).
     * Kernel will inspect this structure before enabling its own paging isolation.
     */
    let physical_memory_offset = 0u64;

    let boot_info_phys = match system_table.boot_services().allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        1,
    ) {
        Ok(address) => address,
        Err(_) => return Status::OUT_OF_RESOURCES,
    };

    let boot_info = BootInfo::new(
        loaded.phys_start,
        loaded.phys_end,
        loaded.entry_point_virt,
        physical_memory_offset,
    );

    unsafe {
        let destination = boot_info_phys as *mut BootInfo;
        destination.write(boot_info);
    }

    let boot_info_ptr = boot_info_phys as *const BootInfo;

    let (_runtime_system_table, _final_memory_map) =
        unsafe { system_table.exit_boot_services(MemoryType::LOADER_DATA) };

    /*
     * Jump to Kernel in Deterministic Identity-Mapped Mode
     */
    let kernel_entry: extern "C" fn(*const BootInfo) -> ! =
        unsafe { core::mem::transmute(loaded.entry_point_virt) };

    kernel_entry(boot_info_ptr);
}

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
