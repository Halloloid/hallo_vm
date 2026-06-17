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

pub enum DecodeError {
    UnknownOpcode(u8),
    Truncated,
}

pub fn encode(op: Op) -> Vec<u8> {
    // its u8 because one byte is 8 bits and its returns array of bytes
    match op {
        Op::Push(n) => {
            let mut bytes = vec![0x01];
            bytes.extend_from_slice(&n.to_le_bytes()); // it helps Split the u64 into its 8 bytes little endian representation
            bytes
        }
        Op::Pop => vec![0x02],
        Op::Dup => vec![0x03],
        Op::Swap => vec![0x04],
        Op::Add => vec![0x10],
        Op::Sub => vec![0x11],
        Op::Mul => vec![0x12],
        Op::Div => vec![0x13],
        Op::Mod => vec![0x14],
        Op::Neg => vec![0x15],
        Op::Load(n) => vec![0x40, n],
        Op::Store(n) => vec![0x41, n],
        Op::Print => vec![0x60],
        Op::Halt => vec![0xFF],
    }
}

pub fn decode(v: &[u8]) -> Result<(Op, u8), DecodeError> {
    match v[0] {
        0x02 => Ok((Op::Pop, 1)),
        0x04 => Ok((Op::Swap, 1)),
        0x03 => Ok((Op::Dup, 1)),
        0x10 => Ok((Op::Add, 1)),
        0x11 => Ok((Op::Sub, 1)),
        0x12 => Ok((Op::Mul, 1)),
        0x13 => Ok((Op::Div, 1)),
        0x14 => Ok((Op::Mod, 1)),
        0x15 => Ok((Op::Neg, 1)),
        0x60 => Ok((Op::Print, 1)),
        0xFF => Ok((Op::Halt, 1)),
        0x01 => {
            if v.len() < 9{
                return Err(DecodeError::Truncated);
            }
            let bytes: [u8; 8] = v[1..9].try_into().unwrap();
            Ok((Op::Push(i64::from_le_bytes(bytes)), 9))
        }
        0x40 => {
            if v.len() < 2{
                return Err(DecodeError::Truncated);
            }
            Ok((Op::Load(v[1]), 2))
        },
        0x41 => {
            if v.len() < 2{
                return Err(DecodeError::Truncated);
            }
            Ok((Op::Store(v[1]), 2))
        },
        _ => Err(DecodeError::UnknownOpcode(v[1])),
    }
}
