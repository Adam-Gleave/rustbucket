; install a 64 bit gdt (created in gdt.rs) to the CPU

global gdt_install
bits 64

gdt_install:
  mov rax, [esp+4]  ; get the pointer to the gdt  passed as parameter in Rust code
  lgdt [rax]        ; load the gdt pointer into cpu

  mov ax, 0x10      ; 0x10 is the offset in the gdt to our data segment
  mov ds, ax        ; Load all data segment selectors
  mov es, ax
  mov fs, ax
  mov gs, ax
  mov ss, ax

  push 0x08  ; jump to code segment
  push .flush
.flush:
  retf ; far return
