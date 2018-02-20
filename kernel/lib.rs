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

	let elf_sections_tag = mb_info.get_elf_sections()
		.expect("ELF sections tag required!");

	let kernel_start = elf_sections_tag.get_sections().map(|s| s.get_start_addr())
	    .min().unwrap();
	let kernel_end = elf_sections_tag.get_sections().map(|s| s.get_start_addr() + s.get_size())
	    .max().unwrap();
	let multiboot_start = mb_info_ptr;
	let multiboot_end = multiboot_start + (mb_info.total_size() as usize);

	vga::clear_term();

  	vga::print("Welcome to the ", 0x07);
  	vga::print("rustbucket", 0x06);
  	vga::println(" kernel!\nStarting boot procedure...");

    write!(Writer::new(), "\nKernel start: {:#X}, kernel end: {:#X}\n", 
	kernel_start, kernel_end);
    write!(Writer::new(), "Multiboot start: {:#X}, Multiboot end: {:#X}\n", 
	multiboot_start, multiboot_end);

	gdt_init();
	idt_init();
	pic_init();
	pit_init(1000);
	
	int::enable();
	vga::println("Enabled interrupts.\n");

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
