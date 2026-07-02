# Folder Structure

```sh
hallovm/
├── Cargo.lock
├── Cargo.toml
├── DESIGN.md
├── README.md
├── src
│   ├── engine
│   │   ├── asm.rs
│   │   ├── dis.rs
│   │   ├── isa.rs
│   │   └── vm.rs
│   ├── engine.rs
│   └── main.rs
└── tests
    ├── integration_test.rs
    ├── program_output
    └── programs
        ├── arith.tasm
        ├── celsius.tasm
        ├── digits.tasm
        ├── horner.tasm
        ├── stackplay.tasm
        ├── trap_divzero.tasm
        ├── trap_overflow.tasm
        ├── trap_truncated.tasm
        ├── trap_underflow.tasm
        └── trap_unknown.tasm

6 directories, 21 files
```