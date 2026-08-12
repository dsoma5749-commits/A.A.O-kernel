#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootInfo {
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
    pub kernel_entry: u64,
    pub physical_memory_offset: u64,

    pub memory_map_phys: u64,
    pub memory_map_size: u64,
    pub memory_map_descriptor_size: u64,
    pub memory_map_descriptor_version: u32,
    pub _reserved: u32,
}

const _: () = {
    assert!(core::mem::size_of::<BootInfo>() == 64);
    assert!(core::mem::align_of::<BootInfo>() == 8);
};
