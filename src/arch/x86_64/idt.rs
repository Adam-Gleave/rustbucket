//idt.rs

//defines the Interrupt Descriptor Table, for use in long mode.

use core::mem::size_of;
use arch::pic;
use driver::vga::Writer;
use core::fmt::Write;
use core::intrinsics::unreachable;
use bochs_break;

const IDT_LENGTH: usize = 256;

extern "C" {
    fn isr_default();
    fn divide_by_zero_wrapper();
    fn breakpoint_wrapper();
}

//contains the structure of an idt entry
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    //base: offset in memory of entry
    base_low: u16,
    //selector: pointer to interrupt handler function
    selector: u16,
    zero1: u8,
    //entry attributes
    flags: u8,
    base_middle: u16,
    base_high: u32,
    zero2: u32
}

//various binary flags that represent entry attributes
enum EntryFlags {
    Present = 0b10000000,
    //flags to determine gate type
    TaskGate = 0b0101,
    InterruptGate = 0b1110,
    TrapGate = 0b1111,
    StorageSeg = 0b10000,
    //flags to determine ring (protection) level
    Ring0 = 0b0000000,
    Ring1 = 0b0100000,
    Ring2 = 0b1000000,
    Ring3 = 0b1100000
}

//contains the pointer to the gdt that must be passed to assembly
#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64
}

//set a static variable containing the IDT pointer
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
impl IdtPointer {
    pub fn new() -> IdtPointer {
        IdtPointer {
            limit: 0,
            base: 0
        }
    }
}

pub struct Idt([IdtEntry; 256]);

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        
        // Exceptions
        idt.set_handler(0, divide_by_zero_wrapper as u64);
        idt.set_handler(1, isr_default as u64);
        idt.set_handler(3, breakpoint_wrapper as u64);

        idt
    };
}

impl Idt {
    pub fn new() -> Idt {
        Idt([IdtEntry::missing(); 256])
    }

    pub fn set_handler(&mut self, vector: u8, func: u64) {
        self.0[vector as usize] = IdtEntry::new(func);
    }

    // pub fn set_handler_with_err(&mut self, vector: u8, func: HandlerErrFunc) {
    //     self.0[vector as usize] = IdtEntry::new(func as u64);
    // }

    pub fn install(&'static self) {
        let mut ptr = IdtPointer::new();
        ptr.limit = (IDT_LENGTH as u16 * size_of::<IdtEntry>() as u16) - 1;
        ptr.base = self as *const _ as u64;
        let location: u64 = ptr.base;

        unsafe {
            asm!("lidt ($0)" :: "r" (&ptr) : "memory");
        }

        write!(Writer::new(), "Success! Created 64-bit IDT at address 0x{:X}\n", ptr.base);
    }
}

impl IdtEntry {
    //constructor, since Rust does not support forward declaration
    pub const fn missing() -> IdtEntry {
        IdtEntry {
            base_low: 0,
            selector: 0,
            zero1: 0,
            flags: 0,
            base_middle: 0,
            base_high: 0,
            zero2: 0
        }
    }

    pub fn new(pointer: u64) -> IdtEntry {
        IdtEntry {
            base_low: pointer as u16,
            base_middle: (pointer >> 16) as u16,
            base_high: (pointer >> 32) as u32,

            selector: 8,

            zero1: 0,
            zero2: 0,

            flags: EntryFlags::InterruptGate as u8 | EntryFlags::Present as u8
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct InterruptFrame {
    pub instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
}

// Initialise IDT
pub fn idt_init() {
    unsafe {
        IDT.install();
    }
}
