# _**halloVM**_

It's a stack-based bytecode VM with its own assembler and disassembler. Write programs in your own assembly language, compile them to a binary format , and execute them on this machine.

## Commands
- `hallovm asm <file.tasm> -o <file.tbc> ` : single-pass assembler with line-numbered errors
- `hallovm run <file.tbc>` : stack machine executing the frozen ISA 
- `hallovm run --trace` : prints ip, instruction, and stack before every step
- `hallovm dis <file.tbc>` : converts back binary to assembler code

## Supported ISAs
`PUSH n`, `POP`, `DUP` ,`SWAP`,`ADD`,`SUB`,`MUL`,`DIV`,`MOD`,`NEG`,`LOAD s`,`STORE s`,`PRINT`,`HALT`

## Limitations
- it does not support comparisons such as : `EQ`, `LT`, `GT`
- it does not support Loops such as : `JMP` , `JZ` , `JNZ` 

