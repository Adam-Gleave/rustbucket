#[macro_use]
pub mod idt;
pub mod int;
pub mod gdt;
pub mod mem;

extern "C" {
    // Default handlers
    fn isr_default();
    fn isr_default_err();
    
    // Exceptions
    fn divide_by_zero_wrapper();
    fn debug_wrapper();
    fn breakpoint_wrapper();
    fn overflow_wrapper();
    fn bounds_wrapper();
    fn opcode_wrapper();
    fn device_na_wrapper();
    fn double_fault_wrapper();
    fn gpf_wrapper();
    fn x87_float_wrapper();
    fn page_fault_wrapper();

    // Interrupts
    fn pit_wrapper();
    fn keyboard_wrapper();
    fn com1_wrapper();
    fn isr_spurious();
}

lazy_static! {
    static ref GDT: gdt::Gdt = {
    	let mut gdt = gdt::Gdt::new();

        //set access flags for code segments
        let code_flags: u8 =
            gdt::AccessFlags::ReadWrite as u8 |
            gdt::AccessFlags::Executable as u8 |
            gdt::AccessFlags::One as u8 |
            gdt::AccessFlags::Present as u8;
        //set access flags for data segments
        let data_flags: u8 =
            gdt::AccessFlags::ReadWrite as u8 |
            gdt::AccessFlags::One as u8 |
            gdt::AccessFlags::Present as u8;
        //set granularity flags, indicate a 64-bit table
        let granularity_flags: u8 =
            gdt::GranularityFlags::Page as u8 |
            gdt::GranularityFlags::LongMode64 as u8;

        gdt.set_entry(0, gdt::GdtEntry::set_up(0, 0, 0, 0));
        gdt.set_entry(1, gdt::GdtEntry::set_up(0, 0xFFFFF, code_flags, granularity_flags));
        gdt.set_entry(2, gdt::GdtEntry::set_up(0, 0xFFFFF, data_flags, granularity_flags));

        gdt
    };
}

lazy_static! {
    static ref IDT: idt::Idt = {
    	let mut idt = idt::Idt::new();

        // Exceptions
        idt.set_handler(0, divide_by_zero_wrapper as u64);
        idt.set_handler(1, debug_wrapper as u64); 
        idt.set_handler(3, breakpoint_wrapper as u64);
        idt.set_handler(4, overflow_wrapper as u64);
        idt.set_handler(5, bounds_wrapper as u64);
        idt.set_handler(6, opcode_wrapper as u64);
        idt.set_handler(7, device_na_wrapper as u64); 
        idt.set_handler(8, double_fault_wrapper as u64);
        idt.set_handler(10, isr_default_err as u64); // Invalid TSS
        idt.set_handler(11, isr_default_err as u64); // Segment not present
        idt.set_handler(12, isr_default_err as u64); // Stack segment fault
        idt.set_handler(13, gpf_wrapper as u64);
        idt.set_handler(14, page_fault_wrapper as u64);
        idt.set_handler(16, x87_float_wrapper as u64);
        idt.set_handler(17, isr_default_err as u64); // Alignment check
        idt.set_handler(18, isr_default as u64); // Machine check
        idt.set_handler(19, isr_default as u64); // SIMD flo{ating point
        idt.set_handler(20, isr_default as u64); // Virtualisation fault
        idt.set_handler(30, isr_default_err as u64); // Security exception

        // Interrupts
        idt.set_handler(32, pit_wrapper as u64);
        idt.set_handler(33, keyboard_wrapper as u64);
        idt.set_handler(36, com1_wrapper as u64);
        idt.set_handler(39, isr_spurious as u64);

        idt
    };
}

pub fn gdt_init() {
    GDT.install();
}

pub fn idt_init() {
    IDT.install();
}
