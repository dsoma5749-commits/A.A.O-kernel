#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MemoryDescriptor {
    pub ty: u32,
    pub _pad: u32,
    pub phys_start: u64,
    pub virt_start: u64,
    pub page_count: u64,
    pub attribute: u64,
}

pub struct BootInfoFrameAllocator {
    memory_map_phys: u64,
    memory_map_size: u64,
    descriptor_size: u64,
    current_byte_offset: u64,
    current_page_in_desc: u64,
}

impl BootInfoFrameAllocator {
    /// Safety: `memory_map_phys` must point to a valid array of UEFI MemoryDescriptors
    pub unsafe fn new(memory_map_phys: u64, memory_map_size: u64, descriptor_size: u64) -> Self {
        Self {
            memory_map_phys,
            memory_map_size,
            descriptor_size,
            current_byte_offset: 0,
            current_page_in_desc: 0,
        }
    }

    /// Allocates a 4KB physical frame from conventional memory (type 7)
    pub fn allocate_frame(&mut self) -> Option<u64> {
        while self.current_byte_offset < self.memory_map_size {
            let desc_ptr =
                (self.memory_map_phys + self.current_byte_offset) as *const MemoryDescriptor;
            let desc = unsafe { &*desc_ptr };

            // MemoryType 7 represents CONVENTIONAL_MEMORY (Free RAM)
            if desc.ty == 7 {
                if self.current_page_in_desc < desc.page_count {
                    let frame_phys = desc.phys_start + (self.current_page_in_desc * 4096);
                    self.current_page_in_desc += 1;
                    return Some(frame_phys);
                }
            }

            self.current_byte_offset += self.descriptor_size;
            self.current_page_in_desc = 0;
        }

        None
    }
}
