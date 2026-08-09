ENTRY(_start)

SECTIONS
{
    . = 0x100000;

    .text : ALIGN(0x1000)
    {
        KEEP(*(.text._start))
        *(.text .text.*)
    }

    .rodata : ALIGN(0x1000)
    {
        *(.rodata .rodata.*)
    }

    .data : ALIGN(0x1000)
    {
        *(.data .data.*)
    }

    .bss : ALIGN(0x1000)
    {
        *(COMMON)
        *(.bss .bss.*)
    }

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment)
    }
}
