//gdt.rs
//defines and provides methods for the Global Descriptor Table, for use in long mode.
//a table has already been defined for protected mode, in boot.asm

use core::mem::size_of;
use core::fmt::Write;
use driver::vga;
use driver::vga::Writer;

const GDT_LENGTH: usize = 3;

//various binary flags that appear in the access field of a gdt entry
//they determine the properties of the entry, and how data is manipulated/accessed
pub enum AccessFlags {
    //for code, indicate it is readable
    //for data, indicate it is writable
    ReadWrite = 0b00000010,
    Executable = 0b00001000, //indicate a code segment
    Present = 0b10000000, //indicate a valid sector
    One = 0b00010000 //self-explanatory -- always set
}

//various binary flags that appear in the granularity field of a gdt entry
pub enum GranularityFlags {
    Page = 0b1000,
    LongMode64 = 0b0010
}

pub struct Gdt([GdtEntry; 3]);

impl Gdt {
    pub fn new() -> Gdt {
        Gdt([GdtEntry::missing(); 3])
    }

    pub fn set_entry(&mut self, vector: u8, entry: GdtEntry) {
       self.0[vector as usize] = entry;
    }

    pub fn install(&'static self) {
        let mut ptr = GdtPointer::new();
        ptr.limit = (GDT_LENGTH as u16 * size_of::<GdtEntry>() as u16) - 1;
        ptr.base = self as *const _ as u64;

        unsafe {
            asm!("lgdt ($0)
                mov $$0x10, %ax
                mov %ax, %ds
                mov %ax, %fs
                mov %ax, %es
                mov %ax, %gs
                mov %ax, %ss" 
                :: "r" (&ptr) : "memory");
        }

        vga::okay();
        unsafe {
            write!(Writer::new(), "Success! Created 64-bit GDT at address 0x{:X}\n", ptr.base)
                .expect("Unexpected failure in write!()");;
        }
    }
}

//contains the structure of a gdt entry
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtEntry {
    //limit: size of entry
    limit_low: u16,
    //base: offset in memory of entry
    base_low: u16,
    base_middle: u8,
    //determine the access level of the segment
    access: u8,
    //determine the granularity of the segment
    //rest of limit stored in granularity byte
    granularity: u8,
    base_high: u8
}

impl GdtEntry {
    pub const fn missing() -> GdtEntry {
        GdtEntry {
            base_low: 0,
            base_middle: 0,
            base_high: 0,
            limit_low: 0,
            granularity: 0,
            access: 0
        }
    }

    pub fn set_up(base_in: u32, limit_in: u32, access_in: u8, gran_in: u8) -> GdtEntry {
        let temp_flags: u8 = ((limit_in >> 16) & 0x0F) as u8;
        let flags: u8 = temp_flags | ((gran_in << 4) &0xF0) as u8;

        GdtEntry {
            //set the base (offset) of the entry
            base_low: base_in as u16,
            base_middle: (base_in >> 16) as u8,
            base_high: (base_in >> 24) as u8,

            //set the size of the entry
            limit_low: limit_in as u16,
            granularity: flags,

            //set the access level of the entry
            access: access_in
        }
    }
}

//contains the pointer to the gdt that must be passed to assembly
#[repr(C, packed)]
pub struct GdtPointer {
    pub limit: u16,
    pub base: u64
}

impl GdtPointer {
    pub fn new() -> GdtPointer {
        GdtPointer {
            limit: 0,
            base: 0
        }
    }
}
