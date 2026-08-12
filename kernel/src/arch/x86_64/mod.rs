pub mod apic;
pub mod context;
pub mod gdt;
pub mod idt;
pub mod paging;
pub mod ring3;
pub mod serial;

pub fn initialize() {
    gdt::init();
    idt::init();
    apic::init_apic_timer();
    serial::init_serial();
}
