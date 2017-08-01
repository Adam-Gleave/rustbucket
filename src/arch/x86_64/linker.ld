ENTRY(start)

SECTIONS {
  /* kernel load address */
  . = 1M;

  .boot :
  {
    /* load multiboot header first */
    KEEP(*(.mboot_h))
  }

  .text :
  {
    /* all input sections named 'text' */
    *(.text)
  }
}
