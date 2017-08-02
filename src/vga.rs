//vga.rs

//contains methods that address the vga buffer of the system
//eg. print_char, print_line, terminal_clear etc

extern crate rlibc;

const COLUMNS: usize = 80;
const ROWS: usize = 25;

pub fn hello_world() {
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
}

pub fn clear_term() {
    //set pointer at start of vga buffer
    let mut buffer = 0xb8000 as *mut _;

    //loop through columns and rows, print whitespace char
    for x in 0..COLUMNS as u8 {
        for y in 0..ROWS as u8 {
            unsafe {
                //shift buffer forward 2 bytes each loop
                buffer = (buffer as u32 + 2) as *mut _;
                //color: 0x0F (black bg, white fg)
                //ascii: 0x20 (whitespace)
                *buffer = 0x0F20
            }
        }
    }
}
