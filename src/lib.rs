// lib.rs

// the root of the cargo project source file
// main kernel operation is here

#![no_std]  //prevent linking of rust std library
#![feature(lang_items)]
#![feature(slice_get_slice)]
#![feature(repr_packed)]

mod vga;
mod arch;

// main kernel function
#[no_mangle] //disbale name mangling (func can be accessed from asm files)
pub extern fn kernel_main() {
	vga::clear_term();

  vga::println("Welcome to the Rustbucket kernel.");
	vga::println("Starting boot procedure...");

	// TODO
	// ----
	// - Create GDT
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
