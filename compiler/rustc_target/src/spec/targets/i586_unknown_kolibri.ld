ENTRY(_start)

__size_of_stack_reserve__ = 1024 * 64;

SECTIONS {
  hdr : {
    LONG(0x554e454D);
    LONG(0x31305445);
    LONG(1);                                               # Header version
    LONG(_start);                                          # Program start
    LONG(END);                                             # End of program code (and data?)
    LONG(___kosapp_memsize);                               # Required amount of memory
    LONG(___kosapp_stack_high);                            # Stack
    LONG(___kosapp_cmdline);                               # Boot params
    LONG(___kosapp_name);                                  # Application path
  }

  .text : {
    *(.init*)
    *(.text)
    *(.text.*)
    *(.fini*)
  }

  .data ALIGN(16) : {
    *(const)
    *(CONST)
    *(.data)
    *(.data.*)
  }

  .rodata ALIGN(16) : {
    *(.rodata*)
  }

  END = .;

  .bss ALIGN(16) : {
    *(.bss*)
    *(COMMON)

    . = ALIGN(16);

    ___kosapp_cmdline = .;
    . = . + 256;
    ___kosapp_name = .;
    . = . + 1024 + 16;
    ___kosapp_stack_low = .;
    . = . + __size_of_stack_reserve__;
    ___kosapp_stack_high = .;
    ___kosapp_memsize = .;
  }

  FILE_END = .;

  /DISCARD/ :
    {
        *(.debug$S)
        *(.debug$T)
        *(.debug$F)
        *(.drectve)
        *(.note.GNU-stack)
        *(.comment)
        *(.debug_abbrev)
        *(.debug_info)
        *(.debug_line)
        *(.debug_frame)
        *(.debug_loc)
        *(.debug_pubnames)
        *(.debug_aranges)
        *(.debug_ranges)
        *(.reloc)
    }
}
