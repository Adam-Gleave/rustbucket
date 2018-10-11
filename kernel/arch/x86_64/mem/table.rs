//table.rs
//contains methods and implementation for page tables

use core::ops::{ Index, IndexMut };
use arch::x86_64::mem::{ entry::*, frame::PageFrame };
use arch::x86_64::mem::ENTRY_COUNT;

pub const P4: *mut Table = 0xffffffff_fffff000 as *mut _;

pub struct Table {
	entries: [Entry; ENTRY_COUNT],
}

impl Index<usize> for Table {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

impl Table {
    pub fn clear(&mut self) {
	for entry in self.entries.iter_mut() {
	    entry.set_as_unused();
	}
    }

    pub fn next_table(&self, index: usize) -> Option<&Table> {
        self.next_table_address(index)
            .map(|address| unsafe {&*(address as *const _)})
    }

    pub fn next_table_as_mut(&mut self, index: usize) -> Option<&mut Table> {
        self.next_table_address(index)
            .map(|address| unsafe {&mut *(address as *mut _)})
    }

    fn next_table_address(&self, index: usize) -> Option<usize> {
        let entry_flags = self[index].flags();
        
        if entry_flags.contains(EntryFlags::PRESENT) 
                && !entry_flags.contains(EntryFlags::HUGE_PAGE) {
            let table_address = self as *const _ as usize;
            Some((table_address << 9) | (index << 12))
        } 
        else {
            None
        }
    }
}
