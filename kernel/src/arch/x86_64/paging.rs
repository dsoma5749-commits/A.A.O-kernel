#![allow(dead_code)]

use x86_64::registers::control::Cr3;
use x86_64::structures::paging::{OffsetPageTable, PageTable, PageTableFlags, PhysFrame};
use x86_64::VirtAddr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UserAddressSpace {
    root_frame: PhysFrame,
}

impl UserAddressSpace {
    pub const fn new(root_frame: PhysFrame) -> Self {
        Self { root_frame }
    }

    pub const fn root_frame(&self) -> PhysFrame {
        self.root_frame
    }

    pub const fn user_flags() -> PageTableFlags {
        PageTableFlags::PRESENT
            .union(PageTableFlags::WRITABLE)
            .union(PageTableFlags::USER_ACCESSIBLE)
    }

    pub const fn kernel_flags() -> PageTableFlags {
        PageTableFlags::PRESENT.union(PageTableFlags::WRITABLE)
    }

    pub const fn executable_user_flags() -> PageTableFlags {
        PageTableFlags::PRESENT.union(PageTableFlags::USER_ACCESSIBLE)
    }
}

/// Access the currently active level-4 page table through a
/// physical-memory identity/offset mapping.
///
/// # Safety
///
/// `physical_memory_offset + CR3.physical_address()` must point to
/// the active level-4 page table and that physical memory must be
/// mapped into the current address space.
pub unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level_4_frame, _) = Cr3::read();

    let physical_address = level_4_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();

    let table_ptr: *mut PageTable = virtual_address.as_mut_ptr();

    &mut *table_ptr
}

/// Create an `OffsetPageTable` over the active address space.
///
/// # Safety
///
/// The caller must guarantee that the supplied physical-memory
/// offset maps all physical memory needed by the page-table hierarchy.
pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);

    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
