//isr.rs

//defines methods for interrupts

use arch::pic;

pub fn enable() {
    unsafe {
        asm!("sti");
    }
}

pub fn disable() {
    unsafe {
        asm!("cli");
    }
}
