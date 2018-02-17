//port_io.rs
//describes methods required to send and receive information from system io ports

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx" ::
        "{dx}"(port), "{al}"(val) ::
        "volatile");
}

#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw %ax, %dx" ::
        "{dx}"(port), "{ax}"(val) ::
        "volatile");
}

#[inline(always)]
pub unsafe fn outl(port: u16, val: u32) {
    asm!("outl %eax, %dx" ::
        "{dx}"(port), "{eax}"(val) ::
        "volatile");
}

#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let val;

    asm!("inb %dx, %al" :
        "={al}"(val) : "{dx}"(port) :
        "al" :
        "volatile");

    val
}

#[inline(always)]
pub unsafe fn inw(port: u16) -> u16 {
    let val;

    asm!("inw %dx, %ax" :
        "={ax}"(val) : "{dx}"(port) :
        "ax" :
        "volatile");

    val
}

#[inline(always)]
pub unsafe fn inl(port: u16) -> u32 {
    let val;

    asm!("inb %dx, %eax" :
        "={eax}"(val) : "{dx}"(port) :
        "eax"
        : "volatile");

    val
}

#[inline(always)]
pub unsafe fn wait() {
    let mut i = 0;

    while i < 150 {
        i = i + 1;
    }
}

#[inline(always)]
pub unsafe fn wait_for(count: u16) {
    let mut i = 0;

    while i < count {
        i = i + 1;
    }
}
