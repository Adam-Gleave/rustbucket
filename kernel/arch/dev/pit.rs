//pit.rs
//contains methods to program the Programmable Interval Timer

use arch::dev::port_io;

pub static mut RATE: u32 = 0;
pub static mut TICKS: u32 = 0;

//port addresses
pub const CMD: u16 = 0x43;
pub const DATA_0: u16 = 0x40;
pub const DATA_1: u16 = 0x41;
pub const DATA_2: u16 = 0x42;

//max phase rate of pit
pub const MAX_RATE: u32 = 1193180;

pub fn set_phase(hz: u32) {
    let divisor = MAX_RATE / hz;

    unsafe {
        RATE = hz;

        port_io::outb(CMD, (3 << 1) | (3 << 4)); //channel 0, least + most significant byte
        port_io::outb(DATA_0, (divisor & 0xFF) as u8); //low
        port_io::outb(DATA_0, (divisor >> 8) as u8); //high
    }
}

pub fn timer_wait(ms: u32) {
    unsafe {
        while TICKS < ms {}
        TICKS = 0;
    }
}
