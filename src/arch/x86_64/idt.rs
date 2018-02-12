//idt.rs

//defines the Interrupt Descriptor Table, for use in long mode.

use core::mem::size_of;
use driver::vga::Writer;
use core::fmt::Write;
use arch::pic;
use bochs_break;
use core::intrinsics::unreachable;

const IDT_LENGTH: usize = 256;

#[macro_export]
macro_rules! save_registers {
    () => {
        #[naked]
        asm!("push rax
            push rcx
            push rdx
            push rsi
            push rdi
            push r8
            push r9
            push r10
            push r11"
            :::: "intel", "volatile");
    }
}

#[macro_export]
macro_rules! restore_registers {
    () => {
        #[naked]
        asm!("pop r11
            pop r10
            pop r9
            pop r8
            pop rdi
            pop rsi
            pop rdx
            pop rcx"
            :::: "intel", "volatile");
    }
}

#[macro_export]
macro_rules! handler {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                save_registers!();
                asm!("mov rdi, rsp
                    sub rsp, 8
                    call $0"
                    :: "i"($name as extern "C" fn(&InterruptFrame))
                    : "rdi" : "intel", "volatile");
                restore_registers!();
                asm!("add rsp, 8
                    iretq"
                    :::: "intel", "volatile");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

#[macro_export]
macro_rules! handler_with_error_code {
    ($name: ident) => {{
        #[naked]
        extern "C" fn wrapper() {
            unsafe {
                asm!("pop rsi
                    mov rdi, rsp
                    sub rsp, 8
                    call $0"
                    :: "i"($name as extern "C" fn(&InterruptFrame, u64) -> !)
                    : "rdi", "rsi" : "intel");
                ::core::intrinsics::unreachable();
            }
        }
        wrapper
    }}
}

//contains the structure of an idt entry
#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IdtEntry {
    //base: offset in memory of entry
    base_low: u16,
    //selector: pointer to interrupt handler function
    selector: u16,
    zero1: u8,
    //entry attributes
    flags: u8,
    base_middle: u16,
    base_high: u32,
    zero2: u32
}

//various binary flags that represent entry attributes
enum EntryFlags {
    Present = 0b10000000,
    //flags to determine gate type
    TaskGate = 0b0101,
    InterruptGate = 0b1110,
    TrapGate = 0b1111,
    StorageSeg = 0b10000,
    //flags to determine ring (protection) level
    Ring0 = 0b0000000,
    Ring1 = 0b0100000,
    Ring2 = 0b1000000,
    Ring3 = 0b1100000
}

//contains the pointer to the gdt that must be passed to assembly
#[repr(C, packed)]
pub struct IdtPointer {
    pub limit: u16,
    pub base: u64
}

//set a static variable containing the IDT pointer
//we use a static variable, since we can find its location in memory with "VAR".as_ptr()
impl IdtPointer {
    pub fn new() -> IdtPointer {
        IdtPointer {
            limit: 0,
            base: 0
        }
    }
}

pub struct Idt([IdtEntry; 256]);

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        
        // Exceptions
        idt.set_handler(0, handler!(divide_by_zero_handler));
        idt.set_handler(1, handler!(debug_handler));
        idt.set_handler(3, handler!(breakpoint_handler));
        idt.set_handler(4, handler!(overflow_handler));
        idt.set_handler(5, handler!(bounds_handler));
        idt.set_handler(6, handler!(invalid_opcode_handler));
        idt.set_handler(7, handler!(device_na_handler));
        idt.set_handler(8, handler_with_error_code!(double_fault_handler));
        idt.set_handler(10, handler_with_error_code!(invalid_tss_handler));
        idt.set_handler(11, handler_with_error_code!(segment_not_present_handler));
        idt.set_handler(12, handler_with_error_code!(stack_segment_fault_handler));
        idt.set_handler(13, handler_with_error_code!(gpf_handler));
        idt.set_handler(14, handler_with_error_code!(page_fault_handler));
        idt.set_handler(16, handler!(x87_floating_point_handler));
        idt.set_handler(17, handler_with_error_code!(alignment_check_handler));
        idt.set_handler(18, handler!(machine_check_handler));
        idt.set_handler(19, handler!(simd_loating_point_handler));
        idt.set_handler(20, handler!(virtualization_handler));
        idt.set_handler(30, handler_with_error_code!(security_handler));

        // Interrupts
        idt.set_handler(33, handler!(keyboard_handler));

        idt
    };
}

impl Idt {
    pub fn new() -> Idt {
        Idt([IdtEntry::missing(); 256])
    }

    pub fn set_handler(&mut self, vector: u8, func: unsafe extern "C" fn()) {
        self.0[vector as usize] = IdtEntry::new(func);
    }

    pub fn install(&'static self) {
        let mut ptr = IdtPointer::new();
        ptr.limit = (IDT_LENGTH as u16 * size_of::<IdtEntry>() as u16) - 1;
        ptr.base = self as *const _ as u64;
        let location: u64 = ptr.base;

        unsafe {
            asm!("lidt ($0)" :: "r" (&ptr) : "memory");
        }

        write!(Writer::new(), "Success! Created 64-bit IDT at address 0x{:X}\n", ptr.base);
    }
}

impl IdtEntry {
    //constructor, since Rust does not support forward declaration
    pub const fn missing() -> IdtEntry {
        IdtEntry {
            base_low: 0,
            selector: 0,
            zero1: 0,
            flags: 0,
            base_middle: 0,
            base_high: 0,
            zero2: 0
        }
    }

    pub fn new(func: unsafe extern "C" fn()) -> IdtEntry {
        let pointer = func as u64;

        IdtEntry {
            base_low: pointer as u16,
            base_middle: (pointer >> 16) as u16,
            base_high: (pointer >> 32) as u32,

            selector: 8,

            zero1: 0,
            zero2: 0,

            flags: EntryFlags::InterruptGate as u8 | EntryFlags::Present as u8
        }
    }
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct InterruptFrame {
    instruction_pointer: u64,
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64
}

// Vector 0
extern "C" fn divide_by_zero_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "Exception: DIVIDE BY ZERO.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 1
extern "C" fn debug_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "EXCEPTION: DEBUG.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 3
extern "C" fn breakpoint_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "EXCEPTION: BREAK POINT.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 4
extern "C" fn overflow_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "EXCEPTION: OVERFLOW.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 5
extern "C" fn bounds_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "Exception: BOUND RANGE EXCEEDED.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 6
extern "C" fn invalid_opcode_handler(frame: &InterruptFrame) {
    write!(Writer::new(),  "EXCEPTION: INVALID OPCODE.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 7
extern "C" fn device_na_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "Exception: DEVICE NOT AVAILABLE.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 8
extern "C" fn double_fault_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: DOUBLE FAULT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}    
}

// Vector 10
extern "C" fn invalid_tss_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: PAGE FAULT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 11
extern "C" fn segment_not_present_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: SEGMENT NOT PRESENT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 12
extern "C" fn stack_segment_fault_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: STACK SEGMENT FAULT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 13
extern "C" fn gpf_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: GENERAL PROTECTION FAULT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 14
extern "C" fn page_fault_handler(frame: &InterruptFrame, code: u64) -> ! {
    write!(Writer::new(),  "EXCEPTION: PAGE FAULT, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 16
extern "C" fn x87_floating_point_handler(frame: &InterruptFrame) {
    write!(Writer::new(),  "EXCEPTION: x87 FLOATING POINT EXCEPTION. \n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 17
extern "C" fn alignment_check_handler(frame: &InterruptFrame, code: u64) -> ! {
    write!(Writer::new(),  "EXCEPTION: ALIGNMENT CHECK, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// Vector 18
extern "C" fn machine_check_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "Exception: MACHINE CHECK.\n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 19
extern "C" fn simd_loating_point_handler(frame: &InterruptFrame) {
    write!(Writer::new(),  "EXCEPTION: SIMD FLOATING POINT EXCEPTION. \n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 20
extern "C" fn virtualization_handler(frame: &InterruptFrame) {
    write!(Writer::new(),  "EXCEPTION: VIRTUALIZATION EXCEPTION. \n\n{:#?}",
        unsafe { &*frame });
    loop {}
}

// Vector 30
extern "C" fn security_handler(frame: &InterruptFrame, code: u64) -> !{
    write!(Writer::new(),  "EXCEPTION: SECURITY EXCEPTION, error code {:?}\n\n{:#?}",
        code, unsafe { &*frame });
    loop {}
}

// PIC vector 1, IDT vector 33
extern "C" fn keyboard_handler(frame: &InterruptFrame) {
    write!(Writer::new(), "Interrupt: Key pressed.");
    loop {}
} 

// Initialise IDT
pub fn idt_init() {
    unsafe {
        IDT.install();
    }
}
