//idt.rs
//defines the Interrupt Descriptor Table, for use in long mode.

use core::mem::size_of;
use core::fmt::Write;
use driver::vga;
use driver::vga::Writer;

const IDT_LENGTH: usize = 256;

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

pub struct Idt([IdtEntry; 256]);

impl Idt {
    pub fn new() -> Idt {
        Idt([IdtEntry::missing(); 256])
    }

    pub fn set_handler(&mut self, vector: u8, func: u64) {
        self.0[vector as usize] = IdtEntry::new(func);
    }

    pub fn install(&'static self) {
        let mut ptr = IdtPointer::new();
        ptr.limit = (IDT_LENGTH as u16 * size_of::<IdtEntry>() as u16) - 1;
        ptr.base = self as *const _ as u64;

        unsafe {
            asm!("lidt ($0)" :: "r" (&ptr) : "memory");
        }

        vga::okay();
        unsafe {
            write!(Writer::new(), "Success! Created 64-bit IDT at address 0x{:X}\n", ptr.base)
                .expect("Unexpected failure in write!()");;
        }
    }
}

//contains the pointer to the gdt that must be passed to assembly
#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64
}

impl IdtPointer {
    pub fn new() -> IdtPointer {
        IdtPointer {
            limit: 0,
            base: 0
        }
    }
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

impl IdtEntry {
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
