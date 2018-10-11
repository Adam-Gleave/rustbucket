//table.rs
//contains methods and implementation for page tables

use core::ops::{ Index, IndexMut }
use arch::x86_64::mem::{ entry::*, frame };

const ENTRIES: usize = 512;

pub struct Table {
	entries: [Entry; ENTRIES],
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
			entry.set_unused();
		}
	}
}
