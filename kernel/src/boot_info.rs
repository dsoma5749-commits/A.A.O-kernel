#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct BootInfo {
    /// Physical address where the kernel image begins.
    pub kernel_phys_start: u64,

    /// Physical address immediately after the kernel image.
    pub kernel_phys_end: u64,

    /// Kernel entry virtual address.
    pub kernel_entry: u64,

    /// Physical-memory mapping offset.
    ///
    /// This is the virtual address corresponding to physical address 0.
    pub physical_memory_offset: u64,
}

impl BootInfo {
    pub const fn new(
        kernel_phys_start: u64,
        kernel_phys_end: u64,
        kernel_entry: u64,
        physical_memory_offset: u64,
    ) -> Self {
        Self {
            kernel_phys_start,
            kernel_phys_end,
            kernel_entry,
            physical_memory_offset,
        }
    }
}
