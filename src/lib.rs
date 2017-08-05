// lib.rs

// the root of the cargo project source file
// main kernel operation is here

#![no_std]  //prevent linking of rust std library
#![allow(dead_code)] //allow unused code in the compiler
#![feature(lang_items)]
#![feature(repr_packed)] //allow structs to be packed in memory
#![feature(asm)] //allow inline assembly
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(const_fn)]

mod driver;
mod arch;

// main kernel function
#[no_mangle] //disbale name mangling (func can be accessed from asm files)
pub extern fn kernel_main() {
	driver::vga::clear_term();

  	driver::vga::println("Welcome to the Rustbucket kernel.");
	driver::vga::println("Starting boot procedure...");

	arch::x86_64::gdt::gdt_init();
	driver::vga::println("\nSuccess! Created 64-bit GDT.");

	// TODO
	// ----
	// - Create IDT
	// - Add exception & hardware interrupt handlers to IDT
	// - Allocate space for thread stacks
	// - Enable PIT (or similar interrupt-driven timer) to preempt threads
	// - Enable interrupts
	// - Halt the CPU until the next timer interrupt occurs, thereby enabling multi-threading

	// EXTRA
	// -----
	// Create dynamic memory allocator
	// Add a keyboard IRQ handler
	// Create a mini kernel-space command-line
	// Begin writing filesystem implementation (filesystems, inodes, file descriptors, etc.)
}

// called on system panic -- not implemented yet
#[lang = "eh_personality"]
extern fn eh_personality() {}

// system panic -- not implemented yet
#[lang = "panic_fmt"]
#[no_mangle]
// ensure the function does not return
pub extern fn panic_fmt() -> ! {
    loop{}
}
