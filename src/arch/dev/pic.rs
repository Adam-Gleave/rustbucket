//pic.rs
//contains methods to configure the PIC, ready for interrupts
//also contains means to communicate with the PIC one it has been initialised

use arch::dev::port_io;
use driver::vga;

//define constants (port numbers)
const PIC_MASTER: u16 = 0x20;
const PIC_SLAVE: u16 = 0xA0;
const PIC_MASTER_COMMAND: u16 = PIC_MASTER;
const PIC_MASTER_DATA: u16 = PIC_MASTER + 1;
const PIC_SLAVE_COMMAND: u16 = PIC_SLAVE;
const PIC_SLAVE_DATA: u16 = PIC_SLAVE + 1;

//code for EOI (end of interrupt) signal
const EOI: u8 = 0x20;

//codes needed for PIC initialisation
const ICW1_ICW4: u8 = 0x01; //states whether ICW4 is needed
const ICW1_SINGLE: u8 = 0x02; //operate in single (cascade) mode
const ICW1_INTERVAL4: u8 = 0x04; //call address interval 4 (8)
const ICW1_LEVEL: u8 = 0x08; //level triggered (edge) mode
const ICW1_INIT: u8 = 0x10; //REQUIRED: start initialisation

//codes needed for further init steps
const ICW4_8086: u8 = 0x01; //8086 mode flag
const ICW4_AUTO: u8 = 0x02; //auto EOI
const ICW4_BUFFER_SLAVE: u8 = 0x08; //buffered mode (slave)
const ICW4_BUFFER_MASTER: u8 = 0x0C; //buffered mode (master)
const ICW4_SFNM: u8 = 0x10; //special (not) fully nested

//codes needed to read ISR/IRR
const OCW3_IRR: u8 = 0x0a;
const OCW3_ISR: u8 = 0x0b;

//PIC vector offsets
const PIC_OFFSET_MASTER: u8 = 32; //offset the PIC indexes by 32
const PIC_OFFSET_SLAVE: u8 = PIC_OFFSET_MASTER + 8;

pub fn init() {
    unsafe {
        //start the initialisation of the PICs
        port_io::outb(PIC_MASTER_COMMAND, ICW1_INIT | ICW1_ICW4);
        port_io::wait();
        port_io::outb(PIC_SLAVE_COMMAND, ICW1_INIT | ICW1_ICW4);
        port_io::wait();

        //provide the PIC vector offsets
        port_io::outb(PIC_MASTER_DATA, PIC_OFFSET_MASTER);
        port_io::wait();
        port_io::outb(PIC_SLAVE_DATA, PIC_OFFSET_SLAVE);
        port_io::wait();

        //provide slave/master relationship information
        port_io::outb(PIC_MASTER_DATA, 4); //inform MASTER there is a SLAVE at IRQ2
        port_io::wait();
        port_io::outb(PIC_SLAVE_DATA, 2); //inform SLAVE it is a cascade identity
        port_io::wait();

        //provide additional environment information
        port_io::outb(PIC_MASTER_DATA, ICW4_8086); //operate in 8086 mode
        port_io::wait();
        port_io::outb(PIC_SLAVE_DATA, ICW4_8086); //operate in 8086 mode
        port_io::wait();

        //mask all interrupts, since none are currently initialised
        port_io::outb(PIC_MASTER_DATA, 0xFF);
        port_io::outb(PIC_SLAVE_DATA, 0xFF);

        irq_set_mask(1, false);
    }

    vga::println("Initialised the PIC, at an offset of 0x20");
}

//send an EOI to the necessary PIC to acknowledge end of interrupt
pub fn ack(irq: u8) {
    unsafe {
        //not a valid irq index
        if irq >= 16 {
            return;
        }

        //irq at slave PIC (PIC2)
        if irq >= 8 {
            port_io::outb(PIC_SLAVE_COMMAND, EOI);
        }
        //must always call EOI at master PIC (PIC1)
        port_io::outb(PIC_MASTER_COMMAND, EOI);
    }
}

//mask or unmask a specific irq in the PIC
pub fn irq_set_mask(mut irq: u8, enable: bool) {
    unsafe {
        let port;
        let value;

        //determine port (PIC) to alter masks at
        if irq < 8 {
            port = PIC_MASTER_DATA;
        } else if irq < 16 {
            port = PIC_SLAVE_DATA;
            irq -= 8;
        } else {
            return; //not a valid irq
        }

        //alter masks accordingly and send to correct PIC
        if !enable {
            value = port_io::inb(port) & !(1 << irq);
            port_io::outb(port, value);
        } else {
            value = port_io::inb(port) | (1 << irq);
            port_io::outb(port, value);
        }
    }
}

fn pic_get_irq_reg(ocw3: u8) -> u16 {
    unsafe {
        port_io::outb(PIC_MASTER_COMMAND, ocw3);
        port_io::outb(PIC_SLAVE_COMMAND, ocw3);
        let val: u16 = ((port_io::inb(PIC_SLAVE_COMMAND) as u16) << 8) |
            port_io::inb(PIC_MASTER_COMMAND) as u16;
        return val;
    }
}

//retrieve the combined IRR registers of the PICs
pub fn pic_get_irr() -> u16 {
    return pic_get_irq_reg(OCW3_IRR);
}

//retrieve the combine ISR registers of the PICs
pub fn pic_get_isr() -> u16 {
    return pic_get_irq_reg(OCW3_ISR);
}
