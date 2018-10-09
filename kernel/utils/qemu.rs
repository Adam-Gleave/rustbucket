// qemu.rs
// utility functions for interacting with the qemu host

use arch::dev::port_io;

// Commonly unused port
const PORT: u16 = 0xF4;

pub fn shutdown() {
    unsafe {
        port_io::outl(PORT, 0);
    }
}

