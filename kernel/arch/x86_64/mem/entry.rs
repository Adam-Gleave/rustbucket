// entry.rs
// Contains structs and methods to represent page table entries

use arch::x86_64::mem::frame::PageFrame;

bitflags! {
    pub struct EntryFlags: u64 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const WRITE_THROUGH = 1 << 3;
        const NO_CACHE = 1 << 4;
        const ACCESSED = 1 << 5;
        const HUGE_PAGE = 1 << 6;
        const DIRTY = 1 << 7;
        const GLOBAL = 1 << 8;
        const NO_EXECUTE = 1 << 63;
    }
}

// Entry contains 64-bit flag
pub struct Entry(u64);

impl Entry {
    pub fn is_unused(&self) -> bool {
        self.0 == 0
    }

    pub fn set_as_unused(&mut self) {
        self.0 = 0;
    }

    pub fn flags(&self) -> EntryFlags {
        EntryFlags::from_bits_truncate(self.0)
    }

    // Get address from page in entry, if present
    pub fn pointed_frame(&self) -> Option<PageFrame> {
        if self.flags().contains(EntryFlags::PRESENT) {
            Some(PageFrame::containing_address(self.0 as usize & 0x000fffff_fffff000))
        }
        else {
            None
        }
    }

    // Set entry to aligned page
    pub fn set(&mut self, frame: PageFrame, flags: EntryFlags) {
        assert!(frame.start() & !0x000fffff_fffff000 == 0);
        self.0 = frame.start() as u64 | flags.bits();
    }
}

