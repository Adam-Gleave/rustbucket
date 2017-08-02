//vga.rs

//contains methods that address the vga buffer of the system
//eg. print_char, print_line, terminal_clear etc

static mut VGA_COL : i32 = 0;
static mut VGA_ROW : i32 = 0;
const VGA_W : i32 = 80;
const VGA_H : i32 = 25;
const VGA_BUFF : usize = 0xB8000;

extern crate rlibc;

pub fn print_char_at(c : u8, x : i32, y : i32, color : u8) {
    let offset : usize = ((y * VGA_W + x) * 2) as usize;
	  let data : i16 = (color as i16) << 8 | (c as i16);
	  unsafe { *((VGA_BUFF + offset) as *mut i16) = data; }
}

pub fn print_char(c : char, color : u8) {
	unsafe {
		match c {
		'\n' =>
			{
				VGA_COL = 0;
				VGA_ROW += 1;
			},
		_ =>
			{
				print_char_at(c as u8, VGA_COL, VGA_ROW, color);
				VGA_COL += 1;
			},
		};

		if VGA_COL >= VGA_W {
			VGA_ROW += 1;
			VGA_COL = 0;
			if VGA_ROW >= VGA_H {
				VGA_ROW = 0;
			}
		}
	}
}

pub fn print_line(str : &str, color : u8) {
	for c in str.chars() {
        print_char(c, color);
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
