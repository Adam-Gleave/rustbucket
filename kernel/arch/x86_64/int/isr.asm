;isr.asm

;defines functions and macros for use handling interrupts
;makes sure all correct information is preserved and restored, and the
;interrupt is passed to the right handler

;default handlers
global isr_default
global isr_default_err

;exceptions
global divide_by_zero_wrapper
global debug_wrapper
global breakpoint_wrapper
global overflow_wrapper
global bounds_wrapper
global opcode_wrapper
global device_na_wrapper
global double_fault_wrapper
global gpf_wrapper
global x87_float_wrapper
global page_fault_wrapper

;interrupts
global pit_wrapper
global keyboard_wrapper
global com1_wrapper
global isr_spurious

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
    sub rsp, 8

    extern isr_default_handler
    call isr_default_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  isr_default_err:
    pop rsi
    mov rdi, rsp
    PUSH_ALL
    sub rsp, 8

    extern isr_default_err_handler
    call isr_default_err_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  isr_spurious:
    PUSH_ALL

    POP_ALL
    iretq

  align 4
  divide_by_zero_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern divide_by_zero_handler
    call divide_by_zero_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  breakpoint_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern breakpoint_handler
    call breakpoint_handler

    add rsp, 8
    POP_ALL
    iretq
  
  align 4
  debug_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern debug_handler
    call debug_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  overflow_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern overflow_handler
    call overflow_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  bounds_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern bounds_handler
    call bounds_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  opcode_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern opcode_handler
    call opcode_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  device_na_wrapper:
    mov rsi, rsp
    sub rsp, 8
    PUSH_ALL
    
    extern device_na_handler
    call device_na_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  double_fault_wrapper:
    pop rsi
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern double_fault_handler
    call double_fault_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  gpf_wrapper:
    pop rsi
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern gpf_handler
    call gpf_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  x87_float_wrapper:
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern x87_float_handler
    call x87_float_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  page_fault_wrapper:
    pop rsi
    mov rdi, rsp
    sub rsp, 8
    PUSH_ALL

    extern page_fault_handler
    call page_fault_handler

    add rsp, 8
    POP_ALL
    iretq

  align 4
  pit_wrapper:
    PUSH_ALL

    extern pit_handler
    call pit_handler

    POP_ALL
    iretq

  align 4
  keyboard_wrapper:
    PUSH_ALL

    extern keyboard_handler
    call keyboard_handler

    POP_ALL
    iretq

  align 4
  com1_wrapper:
    PUSH_ALL

    extern com1_handler
    call com1_handler

    POP_ALL
    iretq

