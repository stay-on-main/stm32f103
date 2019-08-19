MEMORY
{
  FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K   /* 256k */
  RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 20K   /*  32k */
}

SECTIONS
{
  .text ORIGIN(FLASH) :
  {
    /* Vector table */
    LONG(ORIGIN(RAM) + LENGTH(RAM)); /* initial SP value */
    KEEP(*(.reset_handler));

    /* Omitted: The rest of the vector table */

    *(.text.*);
  } > FLASH

  .data ORIGIN(RAM) :
  {
    KEEP(*(.data));
    *(.data.*);
  } > RAM
  
  .bss : ALIGN(4)
  {
    _sbss = .;
    *(.bss .bss.*);
    _ebss = ALIGN(4);
  } > RAM

  /DISCARD/ :
  {
    /* Unused unwinding stuff */
    *(.ARM.exidx.*)
  }
}
