global start          ; global access to kernel entry

section .text
bits 32               ; 32 bit instructions (CPU in protected mode)
start:
  ; print 'OK'
  mov dword [0xb8000], 0x2f4b2f4f
  hlt
