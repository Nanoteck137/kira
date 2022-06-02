ENTRY(_entry)

SECTIONS
{
    . = 0x80000000;
    . = ALIGN(0x1000);
    .text : { *(.text*) }

    . = ALIGN(0x1000);
    .data : { *(.data) *(.data.*)}
    .rodata : { *(.rodata) *(.rodata.*)}
    .bss : { *(.bss) *(.bss.*)}

    _end = .;
}

