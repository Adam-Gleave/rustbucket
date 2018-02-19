//isr.rs
//defines methods for interrupts

use arch::dev::pic;
use arch::dev::pit;
use driver::vga::Writer;
use driver::vga::print_char;
use driver::kbd::get_char;
use core::fmt::Write;

#[derive(Debug)]
#[repr(C)]
pub struct InterruptFrame {
    pub instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
}

// TODO: Bitflags?
#[repr(u64)]
enum PageFaultError {
    ProtectionViolation = 0b00000001,
    AttemptToWrite = 0b00000010,
    UserMode = 0b00000100,
    TableError = 0b00001000,
    InstructionFetch = 0b00010000,
}

#[no_mangle]
#[linkage = "external"]
pub extern fn isr_default_handler(frame: &InterruptFrame) -> ! {
    let frame = &*frame;
    write!(Writer::new(), "EXCEPTION: UNHANDLED EXCEPTION at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame).expect("Unexpected failure in write!()");

    loop {}
}

#[no_mangle]
#[linkage = "external"]
pub extern fn isr_default_err_handler(frame: &InterruptFrame) -> ! {
	let frame = &*frame;
	write!(Writer::new(), "EXCEPTION: UNHANDLED EXCEPTION at instruction {:X}\n{:#?}\n\n",
		frame.instruction_pointer, frame).expect("Unexpected failure in write!()");

	loop {}
}

// Vector 0
#[no_mangle]
#[linkage = "external"]
pub extern fn divide_by_zero_handler(frame: &InterruptFrame) -> ! {
    let frame = &*frame ;
    write!(Writer::new(), "EXCEPTION: DIVIDE BY ZERO at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame).expect("Unexpected failure in write!()");

    loop {}
}

// Vector 3
#[no_mangle]
#[linkage = "external"]
pub extern fn breakpoint_handler(frame: &InterruptFrame) {
    let frame = &*frame;
    write!(Writer::new(), "EXCEPTION: BREAK POINT at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame).expect("Unexpected failure in write!()");
}

// Vector 6
#[no_mangle]
#[linkage = "external"]
pub extern fn opcode_handler(frame: &InterruptFrame) -> ! {
    let frame = &*frame;
    write!(Writer::new(), "EXCEPTION: INVALID OPCODE at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame).expect("Unexpected failure in write!()");

    loop {}
}

// Vector 13
#[no_mangle]
#[linkage = "external"]
pub extern fn double_fault_handler(frame: &InterruptFrame) -> ! {
    let frame = &*frame;
    write!(Writer::new(), "EXCEPTION: DOUBLE FAULT at instruction {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, frame).expect("Unexpected failure in write!()");

    loop {}
}

// Vector 13
#[no_mangle]
#[linkage = "external"]
pub extern fn gpf_handler(frame: &InterruptFrame, code: u64) -> ! {
    let frame = &*frame;
    write!(Writer::new(), "EXCEPTION: GPF at instruction {:#X}, error {:#X}\n{:#?}\n\n",
        frame.instruction_pointer, code, frame).expect("Unexpected failure in write!()");

    loop {}
}

// Vector 14
#[no_mangle]
#[linkage = "external"]
pub extern fn page_fault_handler(frame: &InterruptFrame, code: u64) -> ! {
    let frame = &*frame;
    let code_str;

    // TODO: TIDY UP
    match code {
        code if code == PageFaultError::ProtectionViolation as u64 => code_str = "PROTECTION VIOLATION",
        code if code == PageFaultError::AttemptToWrite as u64 => code_str = "ATTEMPTED TO WRITE",
        code if code == PageFaultError::UserMode as u64 => code_str = "USER MODE",
        code if code == PageFaultError::TableError as u64 => code_str = "ERROR IN TABLE",
        code if code == PageFaultError::InstructionFetch as u64 => code_str = "INSTRUCTION FETCH",
        _ => code_str = "UNKNOWN ERROR",
    }

    write!(Writer::new(), "EXCEPTION: PAGE FAULT at instruction {:#X}, error: {:#}\n{:#?}\n\n",
        frame.instruction_pointer, code_str, frame).expect("Unexpected failure in write!()");

    loop {}
}

// Vector 32
#[no_mangle]
#[linkage = "external"]
pub extern fn pit_handler() {
	pic::ack(1);
    
    unsafe {
        pit::TICKS += 1;

        if pit::TICKS >= 0xFFFFFFFF - 1 {
            pit::TICKS = 0;
        }
    }
}

// Vector 33
#[no_mangle]
#[linkage = "external"]
pub extern fn keyboard_handler() {
    pic::ack(1);
    let c = get_char();

    match c {
        Some(res) => print_char(res, 0x07),
        // Do nothing if NONE returned
        None => {}
    }
}