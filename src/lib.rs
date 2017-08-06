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

use driver::vga::println;
use driver::vga::clear_term;
use arch::x86_64::gdt::gdt_init;
use arch::x86_64::idt::idt_init;
use arch::pic::pic_init;

// main kernel function
#[no_mangle] //disbale name mangling (func can be accessed from asm files)
pub extern fn kernel_main() {
	clear_term();

  	println("Welcome to the Rustbucket kernel.");
	println("Starting boot procedure...");

	//initialise system
	gdt_init(); //set up GDT (global descriptor table)
	idt_init(); //set up IDT (interrupt descriptor table)
	pic_init(); //set up PIC (programmable interrupt controller)

	// TODO
	// ----
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
