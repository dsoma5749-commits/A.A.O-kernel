#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootInfo {
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
    pub kernel_entry: u64,
    pub physical_memory_offset: u64,
    pub memory_map_phys: u64,
    pub memory_map_len: u64,
}

impl BootInfo {
    pub const fn new(
        kernel_phys_start: u64,
        kernel_phys_end: u64,
        kernel_entry: u64,
        physical_memory_offset: u64,
        memory_map_phys: u64,
        memory_map_len: u64,
    ) -> Self {
        Self {
            kernel_phys_start,
            kernel_phys_end,
            kernel_entry,
            physical_memory_offset,
            memory_map_phys,
            memory_map_len,
        }
    }
}

const _: () = {
    assert!(core::mem::size_of::<BootInfo>() == 48);
    assert!(core::mem::align_of::<BootInfo>() == 8);
};
