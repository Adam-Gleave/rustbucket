pub mod frame;
pub mod entry;
pub mod table;

use arch::x86_64::mem::frame::PAGE_SIZE; // needed later

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

