target remote :3333
load
monitor arm semihosting enable
b _start
break main
break main.rs:381
break checkpoint/mod.rs:407