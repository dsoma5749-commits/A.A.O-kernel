#![allow(dead_code)]

pub mod gdt;
pub mod paging;

use x86_64::VirtAddr;

#[derive(Debug)]
pub struct Architecture {
    _private: (),
}

impl Architecture {
    pub const fn new() -> Self {
        Self { _private: () }
    }

    pub fn init(&self) {
        initialize();
    }
}

pub fn initialize() {
    gdt::init();
}

/// Initialize access to the currently active page tables.
///
/// This does not create a new address space yet.
pub unsafe fn init_paging(
    physical_memory_offset: VirtAddr,
) -> x86_64::structures::paging::OffsetPageTable<'static> {
    paging::init(physical_memory_offset)
}

pub use paging::UserAddressSpace;
