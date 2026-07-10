ENTRY(_start)
# this part is used when linked merges all compiled files from .rs to .o to
# a single executable the ELF of that says place all files * matching (.text .text.*)
# in .text section of this single file.
SECTIONS
{
    . = 1M;

    .text : {
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
    }
}