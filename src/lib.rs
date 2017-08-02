// lib.rs

// the root of the cargo project source file
// main kernel operation is here

#![no_std]  //prevent linking of rust std library
#![feature(lang_items)]

// main kernel function
#[no_mangle] //disbale name mangling (func can be accessed from asm files)
pub extern fn kernel_main() {
    let string = b"Hello, World!"; //make a byte array from a string
    let color = 0x1f; //specify colour (blue bg, white fg)
    let mut array = [color; 26];

    //iterate through the string byte array
    //place character in an index (left-most 2 bytes of each 32-bit VGA code)
    for (i, character) in string.into_iter().enumerate() {
        array[i*2] = *character;
    }

    let buffer = 0xb8000 as *mut _; //create a pointer to the VGA buffer location
    unsafe { *buffer = array } //dereference pointer and assign array to location

    loop {}
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
