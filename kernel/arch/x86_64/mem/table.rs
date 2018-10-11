//table.rs
//contains methods and implementation for page tables

use core::marker::PhantomData;
use core::ops::{ Index, IndexMut };
use arch::x86_64::mem::{ entry::*, frame::PageFrame };
use arch::x86_64::mem::ENTRY_COUNT;

pub const P4: *mut Table<Level4> = 0xffffffff_fffff000 as *mut _;

// Table levels
pub trait TableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

// Table levels that are mapped to other tables
// (**NOT** P1)
pub trait MappedLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl MappedLevel for Level4 {
    type NextLevel = Level3;
}

impl MappedLevel for Level3 {
    type NextLevel = Level2;
}

impl MappedLevel for Level2 {
    type NextLevel = Level1;
}

// Phantom data for unused type -- just need level for implementation
pub struct Table<L: TableLevel> {
	entries: [Entry; ENTRY_COUNT],
        level: PhantomData<L>,
}

impl<L> Index<usize> for Table<L> where L: TableLevel {
    type Output = Entry;

    fn index(&self, index: usize) -> &Entry {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel {
    fn index_mut(&mut self, index: usize) -> &mut Entry {
        &mut self.entries[index]
    }
}

impl<L> Table<L> where L: TableLevel {
    pub fn clear(&mut self) {
	for entry in self.entries.iter_mut() {
	    entry.set_as_unused();
	}
    }
}

impl<L> Table<L> where L: MappedLevel {
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.next_table_address(index)
            .map(|address| unsafe {&*(address as *const _)})
    }

    pub fn next_table_as_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
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
