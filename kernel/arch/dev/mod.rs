pub mod pic;
pub mod pit;
pub mod port_io;

use driver::vga;
use core::fmt::Write;
use driver::vga::Writer;

pub fn pic_init() {
    unsafe {
        //start the initialisation of the PICs
        port_io::outb(pic::PIC_MASTER_COMMAND, pic::ICW1_INIT | pic::ICW1_ICW4);
        port_io::wait();
        port_io::outb(pic::PIC_SLAVE_COMMAND, pic::ICW1_INIT | pic::ICW1_ICW4);
        port_io::wait();

        //provide the PIC vector offsets
        port_io::outb(pic::PIC_MASTER_DATA, pic::PIC_OFFSET_MASTER);
        port_io::wait();
        port_io::outb(pic::PIC_SLAVE_DATA, pic::PIC_OFFSET_SLAVE);
        port_io::wait();

        //provide slave/master relationship information
        port_io::outb(pic::PIC_MASTER_DATA, 4); //inform MASTER there is a SLAVE at IRQ2
        port_io::wait();
        port_io::outb(pic::PIC_SLAVE_DATA, 2); //inform SLAVE it is a cascade identity
        port_io::wait();

        //provide additional environment information
        port_io::outb(pic::PIC_MASTER_DATA, pic::ICW4_8086); //operate in 8086 mode
        port_io::wait();
        port_io::outb(pic::PIC_SLAVE_DATA, pic::ICW4_8086); //operate in 8086 mode
        port_io::wait();

        //mask all interrupts, since none are currently initialised
        port_io::outb(pic::PIC_MASTER_DATA, 0xFF);
        port_io::outb(pic::PIC_SLAVE_DATA, 0xFF);

        pic::irq_set_mask(0, false);
        pic::irq_set_mask(1, false);
    }

    vga::okay();
    vga::println("Initialised the PIC, at an offset of 0x20");
}

pub fn pit_init(hz: u32) {
	pit::set_phase(hz);
        vga::okay();
	write!(Writer::new(), "Initialised the PIT, at a phase of {:#} Hz\n", hz)
		.expect("Unexpected failure in write!()");
}
