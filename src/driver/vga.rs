//vga.rs

//contains methods that address the vga buffer of the system
//eg. print_char, print_line, terminal_clear etc

use core::fmt;
use arch::port_io;

static mut VGA_COL: u32 = 0;
static mut VGA_ROW: u32 = 0;
const VGA_W: u32 = 80;
const VGA_H: u32 = 25;
const VGA_BUFF: usize = 0xB8000;

static mut CURSOR_STATE: bool = false;
const CURSOR_LOW_PORT: u16 = 0x3D4;
const CURSOR_HIGH_PORT: u16 = 0x3D5;

extern crate rlibc;

pub fn print_char_at(c: u8, x: u32, y: u32, color: u8) {
    let offset : usize = ((y * VGA_W + x) * 2) as usize;
	let data : u16 = (color as u16) << 8 | (c as u16);
	unsafe { *((VGA_BUFF + offset) as *mut u16) = data; }
}

pub fn print_char(c: char, color: u8) {
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

	cursor_update();
}

pub fn print_byte(c: u8, color: u8) {
	unsafe {
		match c {
		b'\n' =>
			{
				VGA_COL = 0;
				VGA_ROW += 1;
			},
		_ =>
			{
				print_char_at(c, VGA_COL, VGA_ROW, color);
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

	cursor_update();
}

pub fn print(str: &str, color: u8) {
	for c in str.chars() {
        print_char(c, color);
  }
  cursor_update();
}

pub fn println(str: &str) {
	print(str, 0x07);
	print_char('\n', 0x07);
	cursor_update();
}

pub fn clear_term() {
    //loop through columns and rows, print whitespace char
    for x in 0..VGA_W as u32 {
        for y in 0..VGA_H as u32 {
          let offset : usize = ((y * VGA_W + x) * 2) as usize;
          let data : i16 = 0x0720;
          unsafe { *((VGA_BUFF + offset) as *mut i16) = data; }
        }
    }
    cursor_update();
}

// Writer structure, used for write! macro
// enables the use of string formatting for debugging mem addresses, etc
pub struct Writer {

}

impl Writer {
	pub fn new() -> Writer {
		Writer {

		}
	}
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
          print_byte(byte, 0x07);
        }
        cursor_update();

        Ok(())
    }
}

pub fn cursor_enable() {
	unsafe {
		CURSOR_STATE = true;

		port_io::outb(CURSOR_LOW_PORT, 0x0F);
		port_io::outb(CURSOR_HIGH_PORT, (port_io::inb(CURSOR_HIGH_PORT & 0xC0) | 0x0D));
		port_io::outb(CURSOR_LOW_PORT, 0x0B);
		port_io::outb(CURSOR_HIGH_PORT, (port_io::inb(0x3E0 & 0xE0) | 0x0E));

		cursor_update();
	}
}

pub fn cursor_disable() {
	unsafe {
		CURSOR_STATE = false;

		port_io::outb(CURSOR_LOW_PORT, 0x0F);
		port_io::outb(CURSOR_HIGH_PORT, 0x20);
	}
}

fn cursor_update() {
	unsafe {
		if CURSOR_STATE {
				let pos: u16 = (VGA_ROW * VGA_W + VGA_COL) as u16; 

				port_io::outb(0x3D4, 0x0F);
				port_io::outb(0x3D5, (pos & 0xFF) as u8);
				port_io::outb(0x3D4, 0x0E);
				port_io::outb(0x3D5, ((pos >> 8) & 0xFF) as u8);
		}
	}
}
