ENTRY(_start)


# customizing Program Headers to PT_LOAD (load in memory) the specific data (sections) with
# specific file permissions (FLAGS) bootloader (limine) does this loading when sees these PHDRS
PHDRS
{
    text PT_LOAD FLAGS(5);    /* R + X */
    rodata PT_LOAD FLAGS(4);  /* R */
    data PT_LOAD FLAGS(6);    /* R + W */
}


# this part is used when linked merges all compiled files from .rs to .o to
# a single executable the ELF of that says place all files * matching (.text .text.*)
# in .text section of this single file.

# we are specifying 4K page alight for each section cause linker was overlapping sections
# but next section should be in next page not in current even if section lefts some space
SECTIONS
{
    . = 1M;

    .text : ALIGN(4K)
    {
        *(.text .text.*)
    } :text

    . = ALIGN(4K);

    .rodata :
    {
        *(.rodata .rodata.*)
    } :rodata

    . = ALIGN(4K);

    .data :
    {
        *(.data .data.*)
    } :data

    .bss :
    {
        *(.bss .bss.*)
        *(COMMON)
    } :data

    # discards following sections that contain linux metadata somehow
    /DISCARD/ :
    {
        # .eh_frame stores metadata so that rust can unwind when program crashes (panic) but it needs os
        *(.eh_frame*)
        # .note contains unique build identifiers that linux-tools, debug-tools etc use (we don't need that either)
        *(.note*)
    }
}