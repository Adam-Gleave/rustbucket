// kbd.rs

// contains methods that enable the user to perform keyboard input
use arch::port_io;

// Keyboard state modifiers
enum MOD {
	SHIFT = 0,
	CTRL = 1,
	ALT = 2
}

// Modifier table
static MODIFIERS: [bool; 3] = [false, false, false];

// Lookup table, en-us
// TODO: FIX TABLE
static KEYS_EN_US: [u8; 128] = [
	0, 27, '1', '2', '3', '4', '5', '6', '7', '8',
	'9', '0', '-', '=', '\b', 
	'\t', // Tab
	'q', 'w', 'e', 'r',
	't', 'y', 'u', 'i', 'o', 'p', '[', ']', '\n', // Enter key
	0, // 29 - Control
	'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', 
	'\'', '`',
	0,   // Left shift
	'\\', 'z', 'x', 'c', 'v', 'b', 'n',
	'm', ',', '.', '/',
	0,   // Right shift
	'*',
	0,   // Alt
	' ', // Space bar
	0,   // Caps lock
	0,   // 59 - F1 key ... >
	0,   0,   0,   0,   0,   0,   0,   0,
	0,   // < ... F10
	0,   // 69 - Num lock
	0,   // Scroll Lock
	0,   // Home key
	0,   // Up Arrow
	0,   // Page Up
	'-',
	0,   // Left Arrow
	0,
	0,   // Right Arrow
	'+',
	0,   // 79 - End key
	0,   // Down Arrow
	0,   // Page Down
	0,   // Insert Key
	0,   // Delete Key
	0, 0, 0,
	0,   // F11 Key
	0,   // F12 Key
	0, // All other keys are undefined
];

pub fn get_code() {
	let mut c: u8 = 0;
	
	while (port_io::inb(0x64) & 0b00000001) {

	}
}

pub fn get_key() {
	return KEYS_EN_US[get_code() - 1];
}
