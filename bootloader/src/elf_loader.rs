use alloc::vec;
use alloc::vec::Vec;
use uefi::proto::media::file::{File, FileAttribute, FileMode, RegularFile};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::{AllocateType, BootServices, MemoryType};
use uefi::Handle;
use uefi::Status;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Elf64Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub const PT_LOAD: u32 = 1;

pub fn load_kernel_elf(
    image_handle: Handle,
    boot_services: &BootServices,
) -> Result<(u64, Vec<u8>), Status> {
    let mut fs = boot_services
        .get_image_file_system(image_handle)
        .map_err(|_| Status::ABORTED)?;

    let mut root = fs.open_volume().map_err(|_| Status::ABORTED)?;

    let file = root
        .open(
            uefi::cstr16!("kernel.elf"),
            FileMode::Read,
            FileAttribute::READ_ONLY,
        )
        .map_err(|_| Status::NOT_FOUND)?;

    let mut kernel_file = match file.into_regular_file() {
        Some(file) => file,
        None => return Err(Status::ABORTED),
    };

    // Read full ELF into buffer
    let mut file_buf = vec![0u8; 1024 * 1024]; // 1MB initial buffer
    let read_bytes = kernel_file
        .read(&mut file_buf)
        .map_err(|_| Status::ABORTED)?;

    if read_bytes < core::mem::size_of::<Elf64Header>() {
        return Err(Status::LOAD_ERROR);
    }

    let header = unsafe { &*(file_buf.as_ptr() as *const Elf64Header) };

    // Magic verification "\x7FELF"
    if &header.e_ident[0..4] != b"\x7FELF" {
        return Err(Status::LOAD_ERROR);
    }

    let phdr_size = core::mem::size_of::<Elf64Phdr>();
    for i in 0..header.e_phnum {
        let phdr_offset = (header.e_phoff + (i as u64 * phdr_size as u64)) as usize;
        let phdr = unsafe { &*(file_buf.as_ptr().add(phdr_offset) as *const Elf64Phdr) };

        if phdr.p_type == PT_LOAD {
            let pages = ((phdr.p_memsz + 4095) / 4096) as usize;
            let phys_addr = boot_services
                .allocate_pages(
                    AllocateType::Address(phdr.p_paddr),
                    MemoryType::LOADER_DATA,
                    pages,
                )
                .or_else(|_| {
                    boot_services.allocate_pages(
                        AllocateType::AnyPages,
                        MemoryType::LOADER_DATA,
                        pages,
                    )
                })
                .map_err(|_| Status::OUT_OF_RESOURCES)?;

            unsafe {
                let dest =
                    core::slice::from_raw_parts_mut(phys_addr as *mut u8, phdr.p_memsz as usize);
                dest.fill(0);
                let src_offset = phdr.p_offset as usize;
                let filesz = phdr.p_filesz as usize;
                if filesz > 0 {
                    dest[..filesz].copy_from_slice(&file_buf[src_offset..src_offset + filesz]);
                }
            }
        }
    }

    Ok((header.e_entry, file_buf))
}
