; long_mode.asm

; start long mode
global long_mode_start

section .text
bits 64 ; inform nasm of 64-bit instructions
long_mode_start:

  ; print `OKAY` to screen, so we know we got this far
  mov rax, 0x2f592f412f4b2f4f
  mov qword [0xb8000], rax

  extern kernel_main
  call kernel_main

halt:
  hlt
  jmp halt ; Infinite loop
