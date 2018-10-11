pub mod frame;
pub mod entry;
pub mod table;

use arch::x86_64::mem::frame::{Page, PageFrame, PAGE_SIZE};
use arch::x86_64::mem::entry::EntryFlags;
use arch::x86_64::mem::table::P4;

const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub fn translate(virtual_address: VirtualAddress) -> Option<PhysicalAddress> {
    let offset = virtual_address / PAGE_SIZE as usize;

    translate_page(frame::Page::containing_address(virtual_address))
        .map(|frame| frame.number * PAGE_SIZE as usize * offset) 
}

fn translate_page(page: Page) -> Option<PageFrame> {
    let p3 = unsafe { &*P4 }.next_table(page.p4_index());

    let huge_page = || {
            p3.and_then(|p3| {
            let p3_entry = &p3[page.p3_index()];
            // 1GiB page?
            if let Some(start_frame) = p3_entry.pointed_frame() {
                if p3_entry.flags().contains(EntryFlags::HUGE_PAGE) {
                    // address must be 1GiB aligned
                    assert!(start_frame.number % (ENTRY_COUNT * ENTRY_COUNT) == 0);
                    return Some(PageFrame {
                        number: start_frame.number + page.p2_index() *
                                ENTRY_COUNT + page.p1_index(),
                    });
                }
            }
            if let Some(p2) = p3.next_table(page.p3_index()) {
                let p2_entry = &p2[page.p2_index()];
                // 2MiB page?
                if let Some(start_frame) = p2_entry.pointed_frame() {
                    if p2_entry.flags().contains(EntryFlags::HUGE_PAGE) {
                        // address must be 2MiB aligned
                        assert!(start_frame.number % ENTRY_COUNT == 0);
                        return Some(PageFrame {
                            number: start_frame.number + page.p1_index()
                        });
                    }
                }
            }
            None
        })
    };

    p3.and_then(|p3| p3.next_table(page.p3_index()))
      .and_then(|p2| p2.next_table(page.p2_index()))
      .and_then(|p1| p1[page.p1_index()].pointed_frame())
      .or_else(huge_page)
}

