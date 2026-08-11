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

    let physical_memory_offset = 0u64;

    /*
     * Allocate Memory for Memory Map descriptors array and BootInfo
     */
    let memory_map = match system_table
        .boot_services()
        .memory_map(MemoryType::CONVENTIONAL)
    {
        Ok(map) => map,
        Err(_) => return Status::ABORTED,
    };

    let descriptor_count = memory_map.entries().count();
    let map_bytes_len = descriptor_count * core::mem::size_of::<MemoryDescriptor>();
    let map_pages = (map_bytes_len + 4095) / 4096;

    let map_phys_addr = match system_table.boot_services().allocate_pages(
        AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        map_pages,
    ) {
        Ok(address) => address,
        Err(_) => return Status::OUT_OF_RESOURCES,
    };

    unsafe {
        let dest_ptr = map_phys_addr as *mut MemoryDescriptor;
        for (i, desc) in memory_map.entries().enumerate() {
            dest_ptr.add(i).write(*desc);
        }
    }

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
        map_phys_addr,
        descriptor_count as u64,
    );

    unsafe {
        let destination = boot_info_phys as *mut BootInfo;
        destination.write(boot_info);
    }

    let boot_info_ptr = boot_info_phys as *const BootInfo;

    let (_runtime_system_table, _final_memory_map) =
        unsafe { system_table.exit_boot_services(MemoryType::LOADER_DATA) };

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
