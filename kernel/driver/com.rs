// com.rs
// serial port communication

use arch::dev::port_io;
use driver::vga;

const COM1: u16 = 0x3F8;

pub fn init() {
    unsafe {
        // Disable all COM1 interrupts
        port_io::outb(COM1 + 1, 0x0);

        // Enable DLAB (set baud rate divisor)
        port_io::outb(COM1 + 3, 0x80);

        // Set divsor to 3 (38400 baud rate)
        port_io::outb(COM1 + 0, 0x03);
        port_io::outb(COM1 + 1, 0x00);

        // Set data as 8 bits, 1 stop bit, no parity
        port_io::outb(COM1 + 3, 0x03);

        // Enable FIFO, clear with 14-byte threshold
        port_io::outb(COM1 + 2, 0xC7);

        // Enable IRQs, RTS/DSR set
        port_io::outb(COM1 + 4, 0x0B);
    }

    vga::okay();
    vga::println("COM1 serial port initialised");
}

pub fn read() -> u8 {
    unsafe {
        while port_io::inb(COM1 + 5) & 0x01 == 0 {}
        port_io::inb(COM1)
    }
}

pub fn write(byte: u8) {
    unsafe {
        while port_io::inb(COM1 + 5) &0x20 == 0 {}
        port_io::outb(COM1, byte);
    }
}

pub fn write_char(c: char) {
    write(c as u8);
}

pub fn write_str(str: &str) {
    for c in str.chars() {
        write_char(c);
    }
}

