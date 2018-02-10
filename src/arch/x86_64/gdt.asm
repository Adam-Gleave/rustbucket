global gdt_flush

section .text
bits 64

gdt_flush:
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax

    push 0x08
    retfq