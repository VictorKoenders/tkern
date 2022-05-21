ENTRY(0x80000)

PHDRS
{
    segment_code PT_LOAD FLAGS(5); /* RX */
    segment_data PT_LOAD FLAGS(6); /* RW */
}

SECTIONS
{
    . = 0;

    .boot_stack (NOLOAD) :
    {
        /* Stack during the booting process */
        /* The Raspi will locate our kernel at 0x80000 */
        /* We'll use everything before this as our stack */
        . += 0x80000;
    }

    .text :
    {
        __kernel_start = .;
        KEEP(*(.text._start))  /* src/boot.s */
        *(.text._start_rust) /* src/main.rs */
        *(.text*)            /* Everything else */
        . = ALIGN(8);
        __kernel_end = .;
    } :segment_code

    .rodata : ALIGN(8) { *(.rodata*) } :segment_code
    .got    : ALIGN(8) { *(.got)     } :segment_code

    .data : { *(.data*) } :segment_data

    /* Section is zeroed in pairs of u64. Align start and end to 16 bytes */
    .bss (NOLOAD) : ALIGN(16)
    {
        __bss_start = .;
        *(.bss)
        *(.bss*)
        *(COMMON)
        . = ALIGN(16);
        __bss_size = . - __bss_start;
    } :segment_data
}
