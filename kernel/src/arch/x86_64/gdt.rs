#![allow(dead_code)]

use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS, DS, ES, SS};
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::PrivilegeLevel;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

pub static mut TSS: TaskStateSegment = TaskStateSegment::new();

pub struct Selectors {
    pub kernel_code: SegmentSelector,
    pub kernel_data: SegmentSelector,
    pub user_code: SegmentSelector,
    pub user_data: SegmentSelector,
    pub tss: SegmentSelector,
}

pub static mut SELECTORS: Selectors = Selectors {
    kernel_code: SegmentSelector::new(1, PrivilegeLevel::Ring0),
    kernel_data: SegmentSelector::new(2, PrivilegeLevel::Ring0),
    user_code: SegmentSelector::new(3, PrivilegeLevel::Ring3),
    user_data: SegmentSelector::new(4, PrivilegeLevel::Ring3),
    tss: SegmentSelector::new(5, PrivilegeLevel::Ring0),
};

pub fn init() {
    unsafe {
        let tss_ptr = &raw mut TSS;
        (*tss_ptr).interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            let stack_start = x86_64::VirtAddr::from_ptr(&raw const STACK);
            stack_start + STACK_SIZE as u64
        };

        let mut gdt = GlobalDescriptorTable::new();

        let kernel_code = gdt.append(Descriptor::kernel_code_segment());
        let kernel_data = gdt.append(Descriptor::kernel_data_segment());
        let user_code = gdt.append(Descriptor::user_code_segment());
        let user_data = gdt.append(Descriptor::user_data_segment());
        let tss = gdt.append(Descriptor::tss_segment(&*tss_ptr));

        SELECTORS = Selectors {
            kernel_code,
            kernel_data,
            user_code,
            user_data,
            tss,
        };

        static mut GDT_STATIC: GlobalDescriptorTable = GlobalDescriptorTable::new();
        GDT_STATIC = gdt;
        let gdt_ptr = &raw const GDT_STATIC;
        (*gdt_ptr).load();

        CS::set_reg(SELECTORS.kernel_code);
        DS::set_reg(SELECTORS.kernel_data);
        ES::set_reg(SELECTORS.kernel_data);
        SS::set_reg(SELECTORS.kernel_data);

        load_tss(SELECTORS.tss);
    }
}
