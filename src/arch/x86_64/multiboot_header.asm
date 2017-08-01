; multiboot_header.asm

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
