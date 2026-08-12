use core::arch::asm;

pub fn init_serial() {
    unsafe {
        outb(0x3F8 + 1, 0x00); // Disable interrupts
        outb(0x3F8 + 3, 0x80); // Enable DLAB
        outb(0x3F8 + 0, 0x03); // Set divisor to 3 (38400 baud)
        outb(0x3F8 + 1, 0x00);
        outb(0x3F8 + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(0x3F8 + 2, 0xC7); // Enable FIFO
        outb(0x3F8 + 4, 0x0B); // IRQs enabled, RTS/DSR set
    }
}

pub fn print_serial(s: &str) {
    for byte in s.bytes() {
        unsafe {
            while (inb(0x3F8 + 5) & 0x20) == 0 {}
            outb(0x3F8, byte);
        }
    }
}

#[inline]
unsafe fn outb(port: u16, val: u8) {
    asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("in al, dx", out("al") ret, in("dx") port, options(nomem, nostack, preserves_flags));
    ret
}
