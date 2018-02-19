//frame.rs
//contains structs and methods for page frames

const PAGE_SIZE: u16 = 4096;

pub struct PageFrame {
	index: usize,
}

impl PageFrame {
	pub fn at(address: usize) -> PageFrame {
		PageFrame { address / PAGE_SIZE }
	}
}
