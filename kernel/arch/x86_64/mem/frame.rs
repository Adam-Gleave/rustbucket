// frame.rs
// Contains structs and methods for page frames

use arch::x86_64::mem::PhysicalAddress;

pub const PAGE_SIZE: u16 = 4096;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PageFrame(usize);

impl PageFrame {
    // Get the corresponding frame for a physical address
    pub fn at(address: usize) -> PageFrame {
	PageFrame(address / PAGE_SIZE as usize)
    }

    //get the PHYSICAL start address of frame
    pub fn start(&self) -> PhysicalAddress {
	self.0 * PAGE_SIZE as usize
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&self) -> Option<PageFrame>;
    fn deallocate_frame(&self, frame: PageFrame);
}

pub struct Page(usize);

