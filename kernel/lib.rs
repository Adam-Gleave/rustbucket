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
extern crate multiboot2;

mod driver;
mod arch;

use driver::vga;
use driver::vga::Writer;
use core::fmt::Write;
use arch::dev::pic_init;
use arch::dev::pit_init;
use arch::dev::pit;
use arch::x86_64::gdt_init;
use arch::x86_64::idt_init;
use arch::x86_64::int::int;

// called on system panic -- not implemented yet
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
	write!(Writer::new(), "System PANIC at line {}, file \"{}\"", line, file)
		.expect("Unexpected error in write!()");
	write!(Writer::new(), "\t{}", fmt)
		.expect("Unexpected error in write!()");

    loop{}
}

#[no_mangle]
pub extern fn kernel_main(mb_info_ptr: usize) -> ! {
	let mb_info = unsafe { multiboot2::load(mb_info_ptr) };
	let memory_map_tag = mb_info.get_mem_map()
    	.expect("Memory map tag required!");

	vga::clear_term();

  	vga::print("Welcome to the ", 0x07);
  	vga::print("rustbucket", 0x06);
  	vga::println(" kernel!\nStarting boot procedure...");

	write!(Writer::new(), "\n{:?}", memory_map_tag)
		.expect("Unexpected error in write!()");

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
