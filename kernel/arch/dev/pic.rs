//pic.rs
//contains methods to configure the PIC, ready for interrupts
//also contains means to communicate with the PIC one it has been initialised

use arch::dev::port_io;

//define constants (port numbers)
pub const PIC_MASTER: u16 = 0x20;
pub const PIC_SLAVE: u16 = 0xA0;
pub const PIC_MASTER_COMMAND: u16 = PIC_MASTER;
pub const PIC_MASTER_DATA: u16 = PIC_MASTER + 1;
pub const PIC_SLAVE_COMMAND: u16 = PIC_SLAVE;
pub const PIC_SLAVE_DATA: u16 = PIC_SLAVE + 1;

//code for EOI (end of interrupt) signal
pub const EOI: u8 = 0x20;

//codes needed for PIC initialisation
pub const ICW1_ICW4: u8 = 0x01; //states whether ICW4 is needed
pub const ICW1_SINGLE: u8 = 0x02; //operate in single (cascade) mode
pub const ICW1_INTERVAL4: u8 = 0x04; //call address interval 4 (8)
pub const ICW1_LEVEL: u8 = 0x08; //level triggered (edge) mode
pub const ICW1_INIT: u8 = 0x10; //REQUIRED: start initialisation

//codes needed for further init steps
pub const ICW4_8086: u8 = 0x01; //8086 mode flag
pub const ICW4_AUTO: u8 = 0x02; //auto EOI
pub const ICW4_BUFFER_SLAVE: u8 = 0x08; //buffered mode (slave)
pub const ICW4_BUFFER_MASTER: u8 = 0x0C; //buffered mode (master)
pub const ICW4_SFNM: u8 = 0x10; //special (not) fully nested

//codes needed to read ISR/IRR
pub const OCW3_IRR: u8 = 0x0a;
pub const OCW3_ISR: u8 = 0x0b;

//PIC vector offsets
pub const PIC_OFFSET_MASTER: u8 = 32; //offset the PIC indexes by 32
pub const PIC_OFFSET_SLAVE: u8 = PIC_OFFSET_MASTER + 8;

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
