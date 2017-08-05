//idt.rs

//defines the Interrupt Descriptor Table, for use in long mode.

use core::mem::size_of;

const IDT_LENGTH: usize = 256;

//contains the structure of an idt entry
#[derive(Copy, Clone, Debug)]
#[repr(packed)]
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
#[repr(packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64
}

//set a static variable containing the IDT pointer
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
static mut IDT_POINTER: IdtPointer = IdtPointer {
    limit: 0,
    base: 0
};

//set a static variable containing the IDT
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
static mut IDT: [IdtEntry; 256] = [IdtEntry::new(); 256];

impl IdtEntry {
    //constructor, since Rust does not support forward declaration
    pub const fn new() -> IdtEntry {
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
}
