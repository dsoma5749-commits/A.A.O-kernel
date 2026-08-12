#[repr(C)]
pub struct BootInfo {
    pub framebuffer_base: u64,
    pub framebuffer_size: usize,
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub pixels_per_scanline: u32,
    pub memory_map_phys: u64,
    pub memory_map_size: usize,
    pub memory_map_descriptor_size: usize,
    pub memory_map_descriptor_version: u32,
}
