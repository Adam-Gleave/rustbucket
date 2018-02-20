//frame.rs
//contains structs and methods for page frames

const PAGE_SIZE: u16 = 4096;

pub struct PageFrame(usize);

impl PageFrame {
	pub fn at(address: usize) -> PageFrame {
		PageFrame { address / PAGE_SIZE }
	}

	//get the PHYSICAL start address of frame
	pub fn start(&self) -> u64 {
		self.0 * PAGE_SIZE
	}
}

pub trait FrameAllocator {
	pub fn allocate_frame(&self) -> Option<Frame>;
	pub fn deallocate_frame(&self, frame: Frame);
}

pub struct Page(usize);
