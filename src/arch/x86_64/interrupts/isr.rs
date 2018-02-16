//isr.rs

//defines methods for interrupts

use arch::pic;
use driver::vga::Writer;
use driver::vga::print_char;
use driver::kbd::get_char;
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

#[no_mangle]
#[linkage = "external"]
pub extern fn isr_default_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: UNHANDLED EXCEPTION at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    loop {}
}

// Vector 0
#[no_mangle]
#[linkage = "external"]
pub extern fn divide_by_zero_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: DIVIDE BY ZERO at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    loop {}
}

// Vector 3
#[no_mangle]
#[linkage = "external"]
pub extern fn breakpoint_handler(frame: &InterruptFrame) {
    let frame = unsafe { &*frame };
    write!(Writer::new(), "EXCEPTION: BREAK POINT at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame);

    return;
}

// Vector 33
#[no_mangle]
#[linkage = "external"]
pub extern fn keyboard_handler() {
	pic::ack(1);
	let c = get_char();

	match c {
		Some(res) => print_char(res, 0x07),
		None => {}
	}
}
