// lib.rs
// the root of the cargo project source file
// main kernel operation is here

#![no_std]
#![allow(dead_code)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(const_fn)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(linkage)]

#[macro_use]
extern crate lazy_static;

mod driver;
mod arch;
mod multiboot2;

use driver::vga;
use driver::vga::Writer;
use core::fmt::Write;
use arch::dev::pic_init;
use arch::dev::pit_init;
use arch::dev::pit;
use arch::x86_64::gdt_init;
use arch::x86_64::idt_init;
use arch::x86_64::int::int;
use multiboot2::Multiboot2Info;

// called on system panic -- not implemented yet
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt() -> ! {
	vga::println("System panic!");
    loop{}
}

#[no_mangle]
pub extern fn kernel_main(mb_info_ptr: usize) -> ! {
	let mb_info = unsafe { multiboot2::load(mb_info_ptr) };
	let basic_mem = mb_info.get_basic_mem();

	vga::clear_term();

  	vga::print("Welcome to the ", 0x07);
  	vga::print("rustbucket", 0x06);
  	vga::println(" kernel!\nStarting boot procedure...");

  	match basic_mem {
  		Some(res) => {
		  	write!(Writer::new(), "\nMultiboot2 memory info tag:\nmem_lower: {:#X}\nmem_higher: {:#X}\n",
		  		res.mem_lower, res.mem_upper).expect("Unexpected error in write!()");
  		},
  		None => {
		  	write!(Writer::new(), "\nMultiboot2 memory info tag not found")
		  		.expect("Unexpected error in write()!");
  		}
  	}

	gdt_init();
	idt_init();
	pic_init();
	pit_init(1000);
	
	int::enable();
	vga::println("Enabled interrupts.\n");

    pit::timer_wait(2000);
    vga::println("Waited for 2000 milliseconds.");

	loop {}

	// TODO
	// ----
	// - Add exception & hardware interrupt handlers to IDT
	// - Allocate space for thread stacks
	// - Halt the CPU until the next timer interrupt occurs, thereby enabling multi-threading

	// EXTRA
	// -----
	// Create dynamic memory allocator
	// Create a mini kernel-space command-line
	// Begin writing filesystem implementation (filesystems, inodes, file descriptors, etc.)
}

#[naked]
#[inline(always)]
pub fn bochs_break() {
	unsafe {
		asm!("xchg bx, bx" :::: "intel");
	}
}
