target remote :3333

set print asm-demangle on

break DefaultHandler
break HardFault
break rust_begin_unwind

monitor arm semihosting enable
load
continue
quit
