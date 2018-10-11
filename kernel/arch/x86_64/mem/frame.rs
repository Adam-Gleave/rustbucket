// frame.rs
// Contains structs and methods for page frames

use arch::x86_64::mem::{PhysicalAddress, VirtualAddress};

pub const PAGE_SIZE: u16 = 4096;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PageFrame {
    pub number: usize,
}

impl PageFrame {
    // Get the frame containing a virtual address 
    pub fn containing_address(address: PhysicalAddress) -> PageFrame {
        PageFrame { number: address / PAGE_SIZE as usize }   
    }

    //get the PHYSICAL start address of frame
    pub fn start(&self) -> PhysicalAddress {
	self.number * PAGE_SIZE as usize
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&self) -> Option<PageFrame>;
    fn deallocate_frame(&self, frame: PageFrame);
}

pub struct Page(usize);

impl Page {
    // Get page containing virtual address
    pub fn containing_address(address: VirtualAddress) -> Page {
        assert!(address < 0x0000_8000_0000_0000 ||
            address >= 0xffff_8000_0000_0000,
            "invalid address: 0x{:x}", address);
        
        Page (address / PAGE_SIZE as usize)
    }

    fn start(&self) -> usize {
        self.0 * PAGE_SIZE as usize
    }

    pub fn p4_index(&self) -> usize {
        (self.0 >> 27) & 0o777
    }

    pub fn p3_index(&self) -> usize {
        (self.0 >> 18) & 0o777
    }

    pub fn p2_index(&self) -> usize {
        (self.0 >> 9) & 0o777
    }

    pub fn p1_index(&self) -> usize {
        (self.0 >> 0) & 0o777
    }
}

