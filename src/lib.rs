// lib.rs

// the root of the cargo project source file
// main kernel operation is here

#![no_std]  //prevent linking of rust std library
#![feature(lang_items)]

mod vga;

// main kernel function
#[no_mangle] //disbale name mangling (func can be accessed from asm files)
pub extern fn kernel_main() {
    vga::hello_world();
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
