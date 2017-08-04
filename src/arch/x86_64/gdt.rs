//gdt.rs

//defines and provides methods for the Global Descriptor Table, for use in long mode.
//a table has already been defined for protected mode, in boot.asm

//contains the structure of a gdt entry
#[repr(packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    //determine the access level of the segment
    access: u8,
    //determine the granularity of the segment
    granularity: u8,
    base_high: u8
}

//contains the pointer to the gdt that must be passed to assembly
#[repr(packed)]
struct GdtPointer {
    limit: u16,
    base: u32
}

//impl GdtEntry {
//    pub fn new(base_in: u32, limit_in: u32, access_in: u8, gran_in: u8) -> GdtEntry {
//        GdtEntry {
//            //TODO: implement GdtEnrty constructor            
//        }
//    }
//}
