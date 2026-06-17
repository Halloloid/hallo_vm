pub enum Op {
    Push(i64),
    Pop,
    Dup,
    Swap,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Load(u8),
    Store(u8),
    Print,
    Halt,
}

pub fn encode(op: Op) -> Vec<u8> { // its u8 because one byte is 8 bits and its returns array of bytes
    match op {
        Op::Push(n) => {
            let mut bytes = vec![0x01];
            bytes.extend_from_slice(&n.to_le_bytes()); // it helps Split the u64 into its 8 bytes little endian representation
            bytes
        }
        Op::Pop=> vec![0x02],
        Op::Dup=> vec![0x03],
        Op::Swap=> vec![0x04],
        Op::Add => vec![0x10],
        Op::Sub => vec![0x11],
        Op::Mul => vec![0x12],
        Op::Div => vec![0x13],
        Op::Mod => vec![0x14],
        Op::Neg => vec![0x15],
        Op::Load(n) => vec![0x40,n],
        Op::Store(n) => vec![0x41,n],
        Op::Print => vec![0x60],
        Op::Halt => vec![0xFF],
    }
}
