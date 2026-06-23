use crate::engine::{
    helpers::{check_file_length, check_version, file_type_check},
    isa::{DecodeError, decode},
};
use std::{fs, path::Path};

pub fn run(file: &str) {
    let path = Path::new(file);
    let bytes = fs::read(path).expect("Unabel to read The file");
    let mut data = bytes.iter();

    if file_type_check(&mut data)
        && check_version(&mut data)
        && check_file_length(&mut data, bytes.len())
    {
        while let Some(i) = data.next() {
            let mut v = Vec::<u8>::new();
            if *i == 1 {
                let mut c = 8;
                v.push(*i);
                while c != 0 {
                    if let Some(l) = data.next() {
                        v.push(*l);
                    }
                    c -= 1;
                }
            } else if *i == 64 || *i == 65 {
                v.push(*i);
                if let Some(l) = data.next() {
                    v.push(*l);
                }
            } else {
                v.push(*i);
            }

            match decode(&v) {
                Ok((op, _)) => {
                    println!("{}", format!("{:?}", op).to_uppercase());
                }
                Err(DecodeError::Truncated) => {
                    println!("trap : Truncated");
                    break;
                },
                Err(DecodeError::UnknownOpcode(c)) =>{
                    println!("trap at ip={:#06x}: Unknown Opcode",c);
                    break;
                },
            }
        }
    } else {
        println!("Invalid File")
    }
}
