//isr.rs

//defines methods for interrupts

use arch::pic;

#[naked]
#[inline(always)]
pub fn enable() {
    unsafe {
        asm!("sti");
    }
}

#[naked]
#[inline(always)]
pub fn disable() {
    unsafe {
        asm!("cli");
    }
}
