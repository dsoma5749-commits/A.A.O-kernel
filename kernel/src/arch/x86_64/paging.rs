use crate::memory::BootInfoFrameAllocator;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
    pub const PRESENT: u64 = 1 << 0;
    pub const WRITABLE: u64 = 1 << 1;
    pub const USER_ACCESSIBLE: u64 = 1 << 2;

    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_frame(&mut self, frame_phys: u64, flags: u64) {
        self.0 = (frame_phys & 0x000f_ffff_ffff_f000) | flags;
    }

    pub fn frame_address(&self) -> u64 {
        self.0 & 0x000f_ffff_ffff_f000
    }
}

#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [PageTableEntry; 512],
}

impl PageTable {
    pub fn zero(&mut self) {
        for entry in self.entries.iter_mut() {
            entry.0 = 0;
        }
    }
}

pub struct PageTableManager {
    pub l4_table_phys: u64,
}

impl PageTableManager {
    pub fn new(l4_table_phys: u64) -> Self {
        Self { l4_table_phys }
    }

    /// Maps a virtual page (4KB aligned) to a physical frame (4KB aligned)
    pub unsafe fn map_page(
        &mut self,
        virt_addr: u64,
        phys_addr: u64,
        flags: u64,
        allocator: &mut BootInfoFrameAllocator,
    ) -> Result<(), &'static str> {
        let l4_idx = ((virt_addr >> 39) & 0x1ff) as usize;
        let l3_idx = ((virt_addr >> 30) & 0x1ff) as usize;
        let l2_idx = ((virt_addr >> 21) & 0x1ff) as usize;
        let l1_idx = ((virt_addr >> 12) & 0x1ff) as usize;

        let l4 = unsafe { &mut *(self.l4_table_phys as *mut PageTable) };

        let l3_phys = Self::get_or_create_next_table(&mut l4.entries[l4_idx], allocator)?;
        let l3 = unsafe { &mut *(l3_phys as *mut PageTable) };

        let l2_phys = Self::get_or_create_next_table(&mut l3.entries[l3_idx], allocator)?;
        let l2 = unsafe { &mut *(l2_phys as *mut PageTable) };

        let l1_phys = Self::get_or_create_next_table(&mut l2.entries[l2_idx], allocator)?;
        let l1 = unsafe { &mut *(l1_phys as *mut PageTable) };

        l1.entries[l1_idx].set_frame(phys_addr, flags | PageTableEntry::PRESENT);

        Ok(())
    }

    unsafe fn get_or_create_next_table(
        entry: &mut PageTableEntry,
        allocator: &mut BootInfoFrameAllocator,
    ) -> Result<u64, &'static str> {
        if entry.is_unused() {
            let new_frame_phys = allocator
                .allocate_frame()
                .ok_or("Out of physical memory for page table")?;
            let new_table = unsafe { &mut *(new_frame_phys as *mut PageTable) };
            new_table.zero();

            entry.set_frame(
                new_frame_phys,
                PageTableEntry::PRESENT
                    | PageTableEntry::WRITABLE
                    | PageTableEntry::USER_ACCESSIBLE,
            );
            Ok(new_frame_phys)
        } else {
            Ok(entry.frame_address())
        }
    }
}
