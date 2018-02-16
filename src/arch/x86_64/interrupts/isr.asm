;isr.asm

;defines functions and macros for use handling interrupts
;makes sure all correct information is preserved and restored, and the
;interrupt is passed to the right handler

global isr_default

;exceptions
global divide_by_zero_wrapper
global breakpoint_wrapper

;interrupts
global keyboard_wrapper

section .text
bits 64
  ;define a macro for pushing registers onto stack
  %macro PUSH_ALL 0
    push rax
    push rbx
    push rcx
    push rdx
    push rdi
    push rsi
    push rbp
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
  %endmacro

  ;define a macro for restoring values to registers from stack
  %macro POP_ALL 0
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rbp
    pop rsi
    pop rdi
    pop rdx
    pop rcx
    pop rbx
    pop rax
  %endmacro

  align 4
  isr_default:
    mov rdi, rsp
    PUSH_ALL

    extern isr_default_handler
    call isr_default_handler

    POP_ALL
    iretq

  align 4
  divide_by_zero_wrapper:
    mov rdi, rsp
    PUSH_ALL

    extern divide_by_zero_handler
    call divide_by_zero_handler

    POP_ALL
    iretq

  align 4
  breakpoint_wrapper:
    mov rdi, rsp
    PUSH_ALL

    extern breakpoint_handler
    call breakpoint_handler

    POP_ALL
    iretq

  align 4
  keyboard_wrapper:
    PUSH_ALL

    extern keyboard_handler
    call keyboard_handler

    POP_ALL
    iretq
