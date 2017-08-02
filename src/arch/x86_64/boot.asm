; boot.asm

; x86 assembly file, prints "OK" to screen if kernel is compatible
; with the systems CPU

; define constants
global start          ; global access to kernel entry
extern long_mode_start

section .text
bits 32               ; 32 bit instructions (CPU in protected mode)
start:
  mov esp, stack_top

  ; check for incompatibility with processor
                        ; error codes if incompatible
  call check_multiboot  ; throw code 0
  call check_cpuid      ; throw code 1
  call check_long_mode  ; throw code 2

  ; enable paging
  call set_up_paging
  call set_paging

  ; load gdt
  lgdt [gdt64.pointer]
  jmp gdt64.code:long_mode_start

  extern kernel_main
  call kernel_main ; start kernel

  ; print 'OK'
  mov dword [0xb8000], 0x2f4b2f4f ; if this executes, kernel_main() is successful

  hlt

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

; set up paging table references in the cpu
set_up_paging:
  mov eax, p3_table
  or eax, 0b11 ; present + writable flags set
  mov [p4_table], eax

  mov eax, p2_table
  or eax, 0b11
  mov [p3_table], eax

  mov ecx, 0

.map_p2_table:
  ; map P2 entry to a 2MiB huge page
  mov eax, 0x200000  ; 2MiB
  mul ecx            ; start address of nth entry (n stored in ecx)
  or eax, 0b10000011 ; present + writable + huge
  mov [p2_table + ecx * 8], eax ; map nth entry

  inc ecx            ; increase counter
  cmp ecx, 512       ; if counter == 512, the whole P2 table is mapped
  jne .map_p2_table  ; else map the next entry

  ret

; enable paging in the cpu
set_paging:
  ; load p4 table to cr3 register
  ; cpu uses cr3 register to reference p4 tables
  mov eax, p4_table
  mov cr3, eax

  ; set Physical Address Extension (PAE) flag in
  mov eax, cr4
  or eax, 1 << 5
  mov cr4, eax

  ; set long mode bit in MSR
  mov ecx, 0xC0000080
  rdmsr
  or eax, 1 << 8
  wrmsr

  ; enable paging in cr0 register
  mov eax, cr0
  or eax, 1 << 31
  mov cr0, eax

  ret

; stack structure and paging
section .bss
align 4096
p4_table:
  resb 4096
p3_table:
  resb 4096
p2_table:
  resb 4096
stack_bottom:
  resb 16384    ; reserve 16 KB of stack space for kernel
stack_top:

; set up a gdt table (64-bit)
section .read_only:
gdt64:
    dq 0 ; zero entry
.code: equ $ - gdt64 ; store gdt offset
    ; executable, descriptor type, present, 64-bit
    dq (1<<43) | (1<<44) | (1<<47) | (1<<53) ; code segment flags
; create a pointer to the table
.pointer:
    dw $ - gdt64 -1 ; size of table
    dq gdt64 ; address of table

; required data for recognition via bootloader
; mainly magic numbers and necessary data, to enable multiboot support
section .mboot_h
header_start:
  ; definitions
  dd 0xe85250d6                   ; multiboot 2 number
  dd 0                            ; architecture specification: 0 (x86)
  dd header_end - header_start    ; length of header

  ; checksum (avoid compiler warning)
  dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

  ; required multiboot end tags
  dw 0                            ; type
  dw 0                            ; flags
  dw 8                            ; size
header_end:
