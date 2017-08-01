; boot.asm

; x86 assembly file, prints "OK" to screen if kernel is compatible
; with the systems CPU

; define constants
global start          ; global access to kernel entry

section .text
bits 32               ; 32 bit instructions (CPU in protected mode)
start:
  mov esp, stack_top

  ; check for incompatibility with processor
                        ; error codes if incompatible
  call check_multiboot  ; throw code 0
  call check_cpuid      ; throw code 1
  call check_long_mode  ; throw code 2

  extern kernel_main
  call kernel_main ; start kernel

  ; print 'OK'
  mov dword [0xb8000], 0x2f4b2f4f ; if this executes, kernel_main() is successful

  ; system has nothing left to perform:
  ; place system into an infinite loop
  cli        ; clear interrupt flag

back:
  hlt        ; wait for interrupt
  jmp back     ; jump to hlt if system wakes up

; check for multiboot loader
check_multiboot:
    cmp eax, 0x36d76289 ; magic number placed in eax register by loader
    jne .no_multiboot   ; if not in register,
    ret
.no_multiboot:          ; throw error code 0
    mov al, "0"
    jmp error

; detect support for CPUID:
; attempt to flip bit 21 in FLAGS register
; (function from OSDev wiki)
check_cpuid:
  ; copy FLAGS into eax and ecx
  pushfd
  pop eax
  mov ecx, eax

  ; flip bit
  xor eax, 1 << 21

  ; move to FLAGS register
  push eax
  popfd

  ; restore FLAGS to eax (bit will be flipped if CPUID supported)
  ; restore FLAGS before flip op to ecx
  pushfd
  pop eax
  push ecx
  popfd

  ; compare eax to ecx to judge if op successful
  cmp eax, ecx
  je .cpuid_err
  ret

.cpuid_err:
  mov al, "1"
  jmp error

; check 64-bit (long mode) supported
check_long_mode:
  ; check processor age
  mov eax, 0x80000000   ; call cpuid with magic number
  cpuid                 ; place highest argument in eax
  cmp eax, 0x80000001   ; compare to long mode supported cpuid
  jb .no_support

  ; check 64-bit (long mode) support on correct systems
  mov eax, 0x80000001   ; cpuid arg for extended info
  cpuid                 ; place info in ecx and edx
  test edx, 1 << 29     ; check LM bit (bit 29) is set
  jz .no_support
  ret
.no_support:
  mov al, "2"           ; throw error code 2 if no LM support
  jmp error

; print error ("ERR:") and ID character to VGA buffer
; buffer starts at memory address 0xb8000
error:
  mov dword [0xb8000], 0x4f524f45 ; code 4f denotes red background
  mov dword [0xb8004], 0x4f3a4f52
  mov dword [0xb8008], 0x4f204f20
  mov byte  [0xb800a], al
  hlt

; stack structure
section .bss
stack_bottom:
  resb 16384             ; reserve 16 KB of stack space for kernel
stack_top:
