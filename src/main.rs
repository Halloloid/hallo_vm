use std::{
    io::{self, Write, stdout},
    process,
};

mod engine;

fn main() {
    loop {
        print!("$ ");
        stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Unabel to Take the Command");

        let command = command.trim();

        if command == "exit" {
            process::exit(0);
        }

        if !command.starts_with("hallovm") {
            println!("No Such Commands")
        } else {
            let command = &command[8..].trim();

            match command {
                _ if command.starts_with("asm") => {
                    let split: Vec<&str> = command.split("-o").collect();
                    let output_file = split[1].trim();
                    let input_file = split[0][4..].trim();
                    engine::asm::run(input_file, output_file);
                }
                _ => {}
            }
        }
    }
}
