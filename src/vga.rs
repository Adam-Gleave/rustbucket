//vga.rs

//contains methods that address the vga buffer of the system
//eg. print_char, print_line, terminal_clear etc

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

    loop {}
}
