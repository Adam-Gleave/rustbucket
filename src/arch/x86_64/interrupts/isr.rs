//isr.rs

//defines methods for interrupts

use arch::pic;
use driver::vga::Writer;
use core::fmt::Write;
use arch::x86_64::idt::InterruptFrame;

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

#[linkage = "external"]
#[no_mangle] //disable name mangling (func can be accessed from asm files)
pub extern fn isr_default_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: UNHANDLED EXCEPTION at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    loop {}
}

// Vector 0
#[linkage = "external"]
#[no_mangle] //disable name mangling (func can be accessed from asm files)
pub extern fn divide_by_zero_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: DIVIDE BY ZERO at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    loop {}
}

// Vector 3
#[linkage = "external"]
#[no_mangle] //disable name mangling (func can be accessed from asm files)
pub extern fn breakpoint_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: BREAK POINT at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    return;
}
