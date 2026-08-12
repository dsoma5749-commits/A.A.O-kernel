use super::apic::apic_timer_stub;
use core::arch::naked_asm;
use core::ptr::addr_of_mut;

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IdtEntry {
    gdt_selector: u16,
    options: u16,
    offset_low: u16,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            gdt_selector: 0,
            options: 0,
            offset_low: 0,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler_addr: u64) {
        self.gdt_selector = 0x08; // Kernel Code Segment Selector
        self.options = 0x8E00; // Present, Ring 0, Interrupt Gate (0x8E00)
        self.offset_low = handler_addr as u16;
        self.offset_middle = (handler_addr >> 16) as u16;
        self.offset_high = (handler_addr >> 32) as u32;
        self.reserved = 0;
    }
}

#[repr(C, align(16))]
pub struct InterruptDescriptorTable {
    pub entries: [IdtEntry; 256],
}

impl InterruptDescriptorTable {
    pub const fn new() -> Self {
        Self {
            entries: [IdtEntry::missing(); 256],
        }
    }

    pub unsafe fn load(idt_ptr: *const Self) {
        #[repr(C, packed)]
        struct IdtPointer {
            limit: u16,
            base: u64,
        }

        let ptr = IdtPointer {
            limit: (core::mem::size_of::<Self>() - 1) as u16,
            base: idt_ptr as u64,
        };

        core::arch::asm!("lidt [{0}]", in(reg) &ptr, options(readonly, nostack, preserves_flags));
    }
}

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

/// Low-Level Assembly Entry Point for Page Fault Exception (Vector 14)
#[allow(dead_code)]
#[unsafe(naked)]
pub unsafe extern "C" fn page_fault_stub() {
    naked_asm!(
        "cli",
        "push rax",
        "push rcx",
        "push rdx",
        "push rbx",
        "push rbp",
        "push rsi",
        "push rdi",
        "push r8",
        "push r9",
        "push r10",
        "push r11",
        "mov rdi, rsp",
        "call page_fault_handler",
        "pop r11",
        "pop r10",
        "pop r9",
        "pop r8",
        "pop rdi",
        "pop rsi",
        "pop rbp",
        "pop rbx",
        "pop rdx",
        "pop rcx",
        "pop rax",
        "add rsp, 8", // Clean error code off stack
        "iretq"
    );
}

/// C-ABI Page Fault Handler
#[no_mangle]
pub extern "C" fn page_fault_handler(_stack_frame: *const u64) {
    // Handle or recover fault gracefully
}

pub fn init() {
    unsafe {
        let idt_ref = &mut *addr_of_mut!(IDT);
        idt_ref.entries[14].set_handler(page_fault_stub as *const () as u64);
        idt_ref.entries[32].set_handler(apic_timer_stub as *const () as u64); // Vector 32 -> APIC Timer Interrupt Stub
        InterruptDescriptorTable::load(addr_of_mut!(IDT));
    }
}
