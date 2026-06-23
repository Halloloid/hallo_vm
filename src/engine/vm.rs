use std::{fs, path::Path};

use crate::engine::{helpers::{check_file_length, check_version, file_type_check}, isa};

pub fn execute(file: &str, trace: bool) {
    let mut stack = Vec::<i64>::new();
    // let globals: [i64; 256] = [0; 256];
    let mut ip: u8 = 0;

    let path = Path::new(file);
    let bytes = fs::read(path).expect("Unabel to read The File");

    let mut data = bytes.iter();

    if file_type_check(&mut data)
        && check_version(&mut data)
        && check_file_length(&mut data, bytes.len())
    {
        while let Some(byte) = data.next(){
            let mut v = Vec::<u8>::new();
            if *byte == 1{
                let mut c = 8;
                v.push(*byte);
                while c!=0 {
                    if let Some(l) = data.next(){
                        v.push(*l);
                    }
                    c-=1;
                }
            } else {
                v.push(*byte);
            }

            match isa::decode(&v) {
                Ok((op,wide)) => {
                    if trace{
                        println!("{:#06x}\t{}\t{:?}",ip,format!("{:?}",op).to_uppercase(),stack);
                    }
                    match op {
                        isa::Op::Push(n) => {
                            ip+=wide;
                            stack.push(n);
                        },
                        isa::Op::Pop => todo!(),
                        isa::Op::Dup => todo!(),
                        isa::Op::Swap => todo!(),
                        isa::Op::Add => {
                            ip+=wide;
                            let Some(a) = stack.pop() else {
                                eprintln!("trap at ip={:02x}: stack underflow (POP on empty stack)",ip);
                                return;
                            };

                            let Some(b) = stack.pop() else {
                                eprintln!("trap at ip={:02x}: stack underflow (POP on empty stack)",ip);
                                return;
                            };

                            stack.push(b+a);
                        },
                        isa::Op::Sub => todo!(),
                        isa::Op::Mul => todo!(),
                        isa::Op::Div => todo!(),
                        isa::Op::Mod => todo!(),
                        isa::Op::Neg => todo!(),
                        isa::Op::Load(_) => todo!(),
                        isa::Op::Store(_) => todo!(),
                        isa::Op::Print => {
                            ip+=wide;
                            let Some(n) = stack.pop() else {
                                eprintln!("trap at ip={:02x}: stack underflow (POP on empty stack)",ip);
                                return;
                            };
                            println!("{n}");
                        },
                        isa::Op::Halt => {},
                    }
                    
                },
                Err(_) => todo!(),
            }
        }
    }
}
