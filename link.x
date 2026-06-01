ENTRY(_start)

MEMORY {
    RAM (rwx) : ORIGIN = 0x40080000, LENGTH = 64M
}

SECTIONS {
    . = ORIGIN(RAM);

    .text : ALIGN(4) {
        KEEP(*(.text._start))
        *(.text .text.*)
    } > RAM

    .rodata : ALIGN(4) {
        *(.rodata .rodata.*)
    } > RAM

    .data : ALIGN(4) {
        *(.data .data.*)
    } > RAM

    .bss (NOLOAD) : ALIGN(8) {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    .stack (NOLOAD) : ALIGN(8) {
        __stack_bottom = .;
        . = . + 0x4000;
        __stack_top = .;
    } > RAM

    /DISCARD/ : {
        *(.comment .ARM.attributes .note.*)
    }
}