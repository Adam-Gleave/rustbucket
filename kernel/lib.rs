// lib.rs
// the root of the cargo project source file
// main kernel operation is here

#![no_std]
#![allow(dead_code)]
#![feature(panic_implementation)]
#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(naked_functions)]
#![feature(abi_x86_interrupt)]
#![feature(linkage)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate bitflags;
extern crate multiboot2;

mod driver;
mod arch;
mod utils;

use core::intrinsics;
use core::panic::PanicInfo;
use driver::vga;
use driver::vga::Writer;
use driver::com;
use core::fmt::Write;
use arch::dev::pic_init;
use arch::dev::pit_init;
use arch::dev::pit;
use arch::x86_64::gdt_init;
use arch::x86_64::idt_init;
use arch::x86_64::int::int;
use utils::qemu;

// called on system panic -- not implemented yet
#[lang = "eh_personality"]
extern fn eh_personality() {}

#[panic_handler]
#[no_mangle]
pub extern fn panic_fmt(_info: &PanicInfo) -> ! {
    vga::error();
    write!(Writer::new(), " System PANIC!\n")
        .expect("Unexpected error in writing panic information!()");
    vga::error();
    write!(Writer::new(), "{}", _info);

    loop{}
}

#[no_mangle]
pub extern fn kernel_main(mb_info_ptr: usize) -> ! {
    let mb_info = unsafe { multiboot2::load(mb_info_ptr) };

    let elf_sections_tag = mb_info.get_elf_sections()
        .expect("ELF sections tag required!");

    let kernel_start = elf_sections_tag.get_sections().map(|s| s.get_start_addr())
        .min().unwrap();
    let kernel_end = elf_sections_tag.get_sections().map(|s| s.get_start_addr() + s.get_size())
        .max().unwrap();
    let multiboot_start = mb_info_ptr;
    let multiboot_end = multiboot_start + (mb_info.total_size() as usize);

    vga::clear_term();

    vga::print("                     _   _                _        _   
      _ __ _   _ ___| |_| |__  _   _  ___| | _____| |_ 
     | '__| | | / __| __| '_ \\| | | |/ __| |/ / _ \\ __|
     | |  | |_| \\__ \\ |_| |_) | |_| | (__|   <  __/ |_ 
     |_|   \\__,_|___/\\__|_.__/ \\__,_|\\___|_|\\_\\___|\\__|\n\n", 0x06);
                                                   

    vga::print("Welcome to the ", 0x07);
    vga::print("rustbucket", 0x06);
    vga::println(" kernel!\nStarting boot procedure...\n");

    vga::info();
    write!(Writer::new(), "Kernel start: {:#X}, kernel end: {:#X}\n", 
	kernel_start, kernel_end);
    vga::info();
    write!(Writer::new(), "Multiboot start: {:#X}, Multiboot end: {:#X}\n\n", multiboot_start, multiboot_end);

    gdt_init();
    idt_init();
    pic_init();
    pit_init(1000);

    int::enable();
    vga::okay();
    vga::println("Enabled interrupts\n");

    com::init();
    vga::info();
    vga::println("Sending test serial string...\n");
    com::write_str("\nHello from serial!\n");

    vga::info();
    vga::print("Exiting QEMU in ", 0x07);
    
    for i in (1..4).rev() {
        write!(Writer::new(), "{}...", i);
        pit::timer_wait(1000);
    }

    qemu::shutdown();

    //interrupt();

    loop {}

    // TODO
    // ----
    // - Add exception & hardware interrupt handlers to IDT
    // - Allocate space for thread stacks
    // - Halt the CPU until the next timer interrupt occurs, thereby enabling multi-threading

    // EXTRA
    // -----
    // Create dynamic memory allocator
    // Create a mini kernel-space command-line
    // Begin writing filesystem implementation (filesystems, inodes, file descriptors, etc.)
}

#[naked]
#[inline(always)]
pub fn interrupt() {
    unsafe {
        asm!("int3");
    }
}

#[naked]
#[inline(always)]
pub fn bochs_break() {
    unsafe {
    	asm!("xchg bx, bx" :::: "intel");
    }
}
