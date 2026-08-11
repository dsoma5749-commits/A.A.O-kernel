#![allow(dead_code)]

use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

const STACK_SIZE: usize = 4096 * 5;

#[repr(align(16))]
pub struct InterruptStack {
    data: [u8; STACK_SIZE],
}

impl InterruptStack {
    pub const fn new() -> Self {
        Self {
            data: [0; STACK_SIZE],
        }
    }

    pub fn top(&self) -> VirtAddr {
        VirtAddr::from_ptr(self.data.as_ptr().wrapping_add(STACK_SIZE))
    }
}

pub struct TssStorage {
    pub tss: TaskStateSegment,
    pub double_fault_stack: InterruptStack,
}

impl TssStorage {
    pub const fn new() -> Self {
        Self {
            tss: TaskStateSegment::new(),
            double_fault_stack: InterruptStack::new(),
        }
    }

    pub fn initialize(&mut self) {
        let stack_top = self.double_fault_stack.top();

        self.tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_top;
    }
}
