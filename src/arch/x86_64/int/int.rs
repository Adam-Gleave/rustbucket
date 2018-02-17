//int.rs
//interrupt control

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
