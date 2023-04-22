OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80000000;

    .text :
    {
        *(.text.boot)
        *(.text .text.*)
        *(.eh_frame)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data)
    }

    .bss :
    {
        *(.bss)
    }

    . = ALIGN(4k);
    PROVIDE(_stack = .);
}