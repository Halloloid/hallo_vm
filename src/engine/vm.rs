use std::{fs, path::Path};

use crate::engine::{
    helpers::{check_file_length, check_version, file_type_check},
    isa::{Op, decode},
};

pub fn execute(file: &str, trace: bool) -> Result<(), String> {
    let mut stack = Vec::<i64>::new();
    // let mut globals: [i64; 256] = [0; 256];
    let mut ip: u8 = 0;

    let path = Path::new(file);
    let bytes = fs::read(path).expect("Unabel to read The File");

    let mut data = bytes.iter();

    fn pop_stack(stack: &mut Vec<i64>, ip: u8) -> Result<i64, String> {
        stack.pop().ok_or_else(|| {
            format!(
                "trap at ip={:#06x}: stack underflow (POP on empty stack)",
                ip
            )
        })
    }

    if file_type_check(&mut data)
        && check_version(&mut data)
        && check_file_length(&mut data, bytes.len())
    {
        while let Some(byte) = data.next() {
            let mut v = Vec::<u8>::new();
            if *byte == 1 {
                let mut c = 8;
                v.push(*byte);
                while c != 0 {
                    if let Some(l) = data.next() {
                        v.push(*l);
                    }
                    c -= 1;
                }
            } else {
                v.push(*byte);
            }

            match decode(&v) {
                Ok((op, wide)) => {
                    if trace {
                        println!(
                            "{:#06x}   {:<12} {:?}",
                            ip,
                            format!("{:?}", op).to_uppercase(),
                            stack
                        );
                    }
                    match op {
                        Op::Push(n) => stack.push(n),
                        Op::Pop => _ = pop_stack(&mut stack, ip)?,
                        Op::Dup => {
                            let a = pop_stack(&mut stack, ip)?;

                            stack.push(a);
                            stack.push(a);
                        },
                        Op::Swap => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            stack.push(a);
                            stack.push(b);
                        },
                        Op::Add => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            stack.push(b + a);
                        }
                        Op::Sub => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            stack.push(b - a);
                        },
                        Op::Mul => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            stack.push(b * a);
                        },
                        Op::Div => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            if a == 0 {
                                Err(format!("trap at ip={:#06x}: DIV by Zero",ip))?;
                            }
                            
                            stack.push(b / a);
                        },
                        Op::Mod => {
                            let a = pop_stack(&mut stack, ip)?;
                            let b = pop_stack(&mut stack, ip)?;

                            if a == 0 {
                                Err(format!("trap at ip={:#06x}: MOD by Zero",ip))?;
                            }
                            
                            stack.push(b % a);
                        },
                        Op::Neg => {
                            let a = pop_stack(&mut stack, ip)?;
                            stack.push(-a);
                        },
                        Op::Load(_) => todo!(),
                        Op::Store(_) => todo!(),
                        Op::Print => {
                            let n = pop_stack(&mut stack, ip)?;
                            println!("{n}");
                        }
                        Op::Halt => break,
                    }
                    ip += wide;
                }
                Err(_) => todo!(),
            }
        }
    }

    Ok(())
}
