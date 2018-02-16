// kbd.rs

// contains methods that enable the user to perform keyboard input
use arch::port_io;
use driver::vga::Writer;
use core::fmt::Write;

// PS2 port number
const PS2: u16 = 0x60;

// Keyboard state modifiers
enum MOD {
	SHIFT = 0,
	CTRL = 1,
	ALT = 2
}

// Modifier table
static MODIFIERS: [bool; 3] = [false, false, false];

// Scancode set 1 (my keyboard uses this)
pub fn get_char() -> Option<char> {
	let mut code: u8 = 0;
	code = unsafe { port_io::inb(PS2) };

    let result = match code {
        // Alphanumeric
        0x1E => 'a', 0x30 => 'b', 0x2E => 'c', 0x20 => 'd', 0x12 => 'e',
        0x21 => 'f', 0x22 => 'g', 0x23 => 'h', 0x17 => 'i', 0x24 => 'j',
        0x25 => 'k', 0x26 => 'l', 0x32 => 'm', 0x31 => 'n', 0x18 => 'o',
        0x19 => 'p', 0x10 => 'q', 0x13 => 'r', 0x1F => 's', 0x14 => 't',
        0x16 => 'u', 0x2F => 'v', 0x11 => 'w', 0x2D => 'x', 0x15 => 'y',
        0x2C => 'z', 0x0B => '0', 0x02 => '1', 0x03 => '2', 0x04 => '3',
        0x05 => '4', 0x06 => '5', 0x07 => '6', 0x08 => '7', 0x09 => '8',
        0x0A => '9',

        // Symbols
        0x29 => '`', 0x0C => '-', 0x0D => '=', 0x2B => '\\', 0x1A => '[',
        0x1B => ']', 0x27 => ';', 0x28 => '\'', 0x33 => ',', 0x34 => '.',
        0x35 => '/',

        // Keypad
        0x37 => '*', 0x4A => '-', 0x4E => '+', 0x53 => '.',

        // Others
        0x39 => ' ',

        // Undefined
        _ => return None,
    };

    Some(result)
}
