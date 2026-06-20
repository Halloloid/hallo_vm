use crate::engine::isa::{DecodeError, decode};
use std::{fs, path::Path};

pub fn run(file: &str) {
    let path = Path::new(file);
    let bytes = fs::read(path).expect("Unabel to read The file");
    let mut data = bytes.iter();

    let mut type_of_file = Vec::<u8>::new();
    let mut c = 4;
    while c!=0 && let Some(i) = data.next(){
        type_of_file.push(*i);
        c -= 1;
    }

    if type_of_file.len() == 4 {
        if type_of_file[0] == 77
            && type_of_file[1] == 86
            && type_of_file[2] == 77
            && type_of_file[3] == 0
        {
            if let Some(version) = data.next() {
                if *version == 1 {
                    for _ in 0..4 {data.next();} // skiping the length bytes
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
                        } else {
                            v.push(*i);
                        }

                        match decode(&v) {
                            Ok((op, _)) => {
                                println!("{}", format!("{:?}",op).to_uppercase());
                            }
                            Err(DecodeError::Truncated) => todo!(),
                            Err(DecodeError::UnknownOpcode(c)) => {
                                println!("Unknnown Code:{:02x}", c)
                            }
                        }
                    }
                }
            }
        }
    } else {
        println!("Invalid File")
    }
}
