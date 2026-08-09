use core::mem::size_of;
use core::ptr::{copy_nonoverlapping, read_unaligned, write_bytes};

use uefi::table::boot::{
    AllocateType,
    BootServices,
    MemoryDescriptor,
    MemoryType,
};

pub const PT_LOAD: u32 = 1;

const EM_X86_64: u16 = 0x3E;
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const MAX_LOAD_SEGMENTS: usize = 8;
const PAGE_SIZE: u64 = 4096;

#[repr(C)]
#[derive(Clone, Copy)]
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
}

#[repr(C)]
#[derive(Clone, Copy)]
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

#[allow(dead_code)]
pub struct LoadedKernelInfo {
    pub phys_start: u64,
    pub phys_end: u64,
    pub entry_point_virt: u64,
}

pub unsafe fn parse_and_load_kernel(
    elf_bytes: &[u8],
    memory_map: &[MemoryDescriptor],
    boot_services: &BootServices,
) -> Result<LoadedKernelInfo, &'static str> {
    if elf_bytes.len() < size_of::<Elf64Header>() {
        return Err("ELF_FILE_TOO_SMALL");
    }

    let header =
        read_unaligned(elf_bytes.as_ptr() as *const Elf64Header);

    if header.e_ident[0] != 0x7F
        || header.e_ident[1] != b'E'
        || header.e_ident[2] != b'L'
        || header.e_ident[3] != b'F'
    {
        return Err("INVALID_ELF_MAGIC");
    }

    if header.e_ident[4] != ELFCLASS64 {
        return Err("ELF_NOT_64_BIT");
    }

    if header.e_ident[5] != ELFDATA2LSB {
        return Err("ELF_NOT_LITTLE_ENDIAN");
    }

    if header.e_machine != EM_X86_64 {
        return Err("NOT_X86_64_ELF");
    }

    if header.e_version != 1 {
        return Err("INVALID_ELF_VERSION");
    }

    if header.e_ehsize as usize != size_of::<Elf64Header>() {
        return Err("INVALID_ELF_HEADER_SIZE");
    }

    if header.e_phentsize as usize != size_of::<Elf64Phdr>() {
        return Err("INVALID_PROGRAM_HEADER_SIZE");
    }

    let phdr_offset = usize::try_from(header.e_phoff)
        .map_err(|_| "PHDR_OFFSET_OVERFLOW")?;

    let phdr_size = usize::from(header.e_phentsize);
    let phdr_count = usize::from(header.e_phnum);

    let phdr_table_size = phdr_count
        .checked_mul(phdr_size)
        .ok_or("PHDR_TABLE_SIZE_OVERFLOW")?;

    let phdr_end = phdr_offset
        .checked_add(phdr_table_size)
        .ok_or("PHDR_TABLE_END_OVERFLOW")?;

    if phdr_end > elf_bytes.len() {
        return Err("PHDR_TABLE_OUT_OF_BOUNDS");
    }

    let mut ranges = [(0u64, 0u64); MAX_LOAD_SEGMENTS];
    let mut range_count = 0usize;

    let mut min_phys = u64::MAX;
    let mut max_phys = 0u64;
    let mut found_load = false;

    /*
     * First pass:
     * validate every PT_LOAD and calculate total physical range.
     */
    for index in 0..phdr_count {
        let offset = index
            .checked_mul(phdr_size)
            .and_then(|v| phdr_offset.checked_add(v))
            .ok_or("PHDR_OFFSET_OVERFLOW")?;

        let phdr =
            read_unaligned(elf_bytes.as_ptr().add(offset) as *const Elf64Phdr);

        if phdr.p_type != PT_LOAD {
            continue;
        }

        found_load = true;

        if range_count >= MAX_LOAD_SEGMENTS {
            return Err("TOO_MANY_LOADABLE_SEGMENTS");
        }

        if phdr.p_filesz > phdr.p_memsz {
            return Err("FILESZ_GREATER_THAN_MEMSZ");
        }

        if phdr.p_align != 0 && !phdr.p_align.is_power_of_two() {
            return Err("INVALID_SEGMENT_ALIGNMENT");
        }

        if phdr.p_align > 1
            && phdr.p_vaddr % phdr.p_align != phdr.p_offset % phdr.p_align
        {
            return Err("SEGMENT_ALIGNMENT_MISMATCH");
        }

        let file_end = phdr
            .p_offset
            .checked_add(phdr.p_filesz)
            .ok_or("SEGMENT_FILE_RANGE_OVERFLOW")?;

        if file_end > elf_bytes.len() as u64 {
            return Err("SEGMENT_FILE_RANGE_OUT_OF_BOUNDS");
        }

        let mem_end = phdr
            .p_paddr
            .checked_add(phdr.p_memsz)
            .ok_or("PHYSICAL_ADDRESS_OVERFLOW")?;

        if phdr.p_memsz == 0 {
            continue;
        }

        for &(start, end) in &ranges[..range_count] {
            if phdr.p_paddr < end && mem_end > start {
                return Err("KERNEL_SEGMENTS_OVERLAP");
            }
        }

        ranges[range_count] = (phdr.p_paddr, mem_end);
        range_count += 1;

        if phdr.p_paddr < min_phys {
            min_phys = phdr.p_paddr;
        }

        if mem_end > max_phys {
            max_phys = mem_end;
        }

        if !is_region_available_in_uefi(
            phdr.p_paddr,
            phdr.p_memsz,
            memory_map,
        ) {
            return Err("TARGET_PHYSICAL_MEMORY_NOT_CONVENTIONAL");
        }
    }

    if !found_load {
        return Err("ELF_HAS_NO_PT_LOAD_SEGMENTS");
    }

    if min_phys == u64::MAX || max_phys <= min_phys {
        return Err("INVALID_KERNEL_PHYSICAL_RANGE");
    }

    if header.e_entry == 0 {
        return Err("INVALID_KERNEL_ENTRY_POINT");
    }

    /*
     * The kernel uses fixed physical addresses.
     *
     * Reserve the complete [min_phys, max_phys) range before copying.
     */
    let alloc_start = min_phys & !(PAGE_SIZE - 1);
    let alloc_end = (max_phys + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);

    if alloc_end <= alloc_start {
        return Err("INVALID_ALLOCATION_RANGE");
    }

    let alloc_size = alloc_end
        .checked_sub(alloc_start)
        .ok_or("ALLOCATION_RANGE_OVERFLOW")?;

    let page_count =
        usize::try_from(alloc_size / PAGE_SIZE)
            .map_err(|_| "PAGE_COUNT_OVERFLOW")?;

    boot_services
        .allocate_pages(
            AllocateType::Address(alloc_start),
            MemoryType::LOADER_DATA,
            page_count,
        )
        .map_err(|_| "KERNEL_MEMORY_ALLOCATION_FAILED")?;

    /*
     * Second pass:
     * copy each PT_LOAD into its reserved physical address.
     */
    for index in 0..phdr_count {
        let offset = index
            .checked_mul(phdr_size)
            .and_then(|v| phdr_offset.checked_add(v))
            .ok_or("PHDR_OFFSET_OVERFLOW")?;

        let phdr =
            read_unaligned(elf_bytes.as_ptr().add(offset) as *const Elf64Phdr);

        if phdr.p_type != PT_LOAD || phdr.p_memsz == 0 {
            continue;
        }

        let destination = phdr.p_paddr as *mut u8;

        if phdr.p_filesz != 0 {
            let source =
                elf_bytes.as_ptr().add(phdr.p_offset as usize);

            copy_nonoverlapping(
                source,
                destination,
                phdr.p_filesz as usize,
            );
        }

        let bss_size = phdr.p_memsz - phdr.p_filesz;

        if bss_size != 0 {
            write_bytes(
                destination.add(phdr.p_filesz as usize),
                0,
                bss_size as usize,
            );
        }
    }

    Ok(LoadedKernelInfo {
        phys_start: min_phys,
        phys_end: max_phys,
        entry_point_virt: header.e_entry,
    })
}

fn is_region_available_in_uefi(
    start: u64,
    size: u64,
    memory_map: &[MemoryDescriptor],
) -> bool {
    let end = match start.checked_add(size) {
        Some(value) => value,
        None => return false,
    };

    for descriptor in memory_map {
        if descriptor.ty != MemoryType::CONVENTIONAL {
            continue;
        }

        let region_size = match descriptor.page_count.checked_mul(4096) {
            Some(value) => value,
            None => continue,
        };

        let region_end =
            match descriptor.phys_start.checked_add(region_size) {
                Some(value) => value,
                None => continue,
            };

        if start >= descriptor.phys_start && end <= region_end {
            return true;
        }
    }

    false
}
