// kbd.rs
// contains methods that enable the user to perform keyboard input

use arch::dev::port_io;

const PS2: u16 = 0x60;

enum MOD {
	SHIFT = 0,
	CTRL = 1,
	ALT = 2
}

static mut MODIFIERS: [bool; 3] = [false, false, false];

// Scancode set 1 (my keyboard uses this)
pub fn get_char() -> Option<char> {
    let code = unsafe { port_io::inb(PS2) };
    set_mods(code);

    code_to_char(code)
}

fn set_mods(code: u8) {
    unsafe {
        match code {
            0x2A | 0x36 => MODIFIERS[0] = true,
            0xAA | 0xB6 => MODIFIERS[0] = false,
            0x3A => MODIFIERS[0] = !MODIFIERS[0],
            
            _ => {},
        }
    }
}

fn code_to_char(code: u8) -> Option<char> {
    let result;

    unsafe {
        if MODIFIERS[0] {
            result = get_shift(code);
        } else {
            result = get_reg(code);
        }
    }

    result
}

fn get_shift(code: u8) -> Option<char> {
    let result = match code {
        // Alphanumeric
        0x1E => 'A', 0x30 => 'B', 0x2E => 'C', 0x20 => 'D', 0x12 => 'E',
        0x21 => 'F', 0x22 => 'G', 0x23 => 'H', 0x17 => 'I', 0x24 => 'J',
        0x25 => 'K', 0x26 => 'L', 0x32 => 'M', 0x31 => 'N', 0x18 => 'O',
        0x19 => 'P', 0x10 => 'Q', 0x13 => 'R', 0x1F => 'S', 0x14 => 'T',
        0x16 => 'U', 0x2F => 'V', 0x11 => 'W', 0x2D => 'X', 0x15 => 'Y',
        0x2C => 'Z', 0x0B => ')', 0x02 => '!', 0x03 => '@', 0x04 => '#',
        0x05 => '$', 0x06 => '%', 0x07 => '^', 0x08 => '&', 0x09 => '*',
        0x0A => '(',

        // Symbols
        0x29 => '~', 0x0C => '_', 0x0D => '+', 0x2B => '|', 0x1A => '{',
        0x1B => '}', 0x27 => ':', 0x28 => '"', 0x33 => '<', 0x34 => '>',
        0x35 => '?',

        // Keypad
        0x37 => '*', 0x4A => '-', 0x4E => '+', 0x53 => '.',

        // Others
        0x39 => ' ', 0x0F => '\t', 0x1C => '\n', 0x0E => '\u{8}',

        // Undefined
        _ => return None,
    };

    Some(result)
}

fn get_reg(code: u8) -> Option<char> {
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
        0x39 => ' ', 0x0F => '\t', 0x1C => '\n', 0x0E => '\u{8}',

        // Undefined
        _ => return None,
    };

    Some(result)
}
