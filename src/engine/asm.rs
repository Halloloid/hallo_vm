use crate::engine::isa::{Op, encode};
use std::{fs, path::Path};

pub fn run(file_to_read: &str, file_to_write: &str) {
    let path = Path::new(file_to_read);
    let path_write = Path::new(file_to_write);
    let data = fs::read_to_string(path).expect("Unabel to read The file");
    let cmds = to_asm_cmds(data);
    let mut v: Vec<u8> = Vec::new();
    v.extend([0x4D,0x56,0x4D,0x00]);
    v.push(0x01); // as its my first version
    v.push(cmds.len() as u8);
    for i in cmds {
        match i.as_str() {
            "POP" => v.push(encode(Op::Pop)[0]),
            "DUP" => v.push(encode(Op::Dup)[0]),
            "SWAP" => v.push(encode(Op::Swap)[0]),
            "ADD" => v.push(encode(Op::Add)[0]),
            "SUB" => v.push(encode(Op::Sub)[0]),
            "MUL" => v.push(encode(Op::Mul)[0]),
            "DIV" => v.push(encode(Op::Div)[0]),
            "MOD" => v.push(encode(Op::Mod)[0]),
            "NEG" => v.push(encode(Op::Neg)[0]),
            "PRINT" => v.push(encode(Op::Print)[0]),
            "HALT" => v.push(encode(Op::Halt)[0]),
            k if k.starts_with("PUSH") => {
                let idx: Vec<&str> = k.split(' ').collect();
                let n = idx[1].parse::<i64>().unwrap();
                let vec = encode(Op::Push(n));
                v.extend(vec);
            }
            k if k.starts_with("LOAD") => {
                let idx: Vec<&str> = k.split(' ').collect();
                let n = idx[1].parse::<u8>().unwrap();
                let vec = encode(Op::Load(n));
                v.extend(vec);
            }
            k if k.starts_with("STORE") => {
                let idx: Vec<&str> = k.split(' ').collect();
                let n = idx[1].parse::<u8>().unwrap();
                let vec = encode(Op::Store(n));
                v.extend(vec);
            }
            _ => {}
        }
    }

    fs::write(path_write, v).expect("Unabel to Write");
}

fn to_asm_cmds(data: String) -> Vec<String> {
    let mut v = Vec::new();
    for line in data.lines() {
        if line.contains(';') {
            let cmd: Vec<&str> = line.split(';').collect();
            let cmd = cmd[0].trim();
            if !cmd.is_empty() {
                v.push(format!("{cmd}"));
            }
        } else {
            if !line.is_empty() {
                v.push(format!("{}", line.trim()));
            }
        }
    }
    v
}
