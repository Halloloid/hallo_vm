use std::{fs,path::Path};

use crate::engine::helpers::file_type_check;

pub fn execute(file:&str,trace:bool){
    let mut stack = Vec::<i64>::new();
    let globals:[i64;256] = [0;256];
    let mut ip:u8 = 0;

    let path = Path::new(file);
    let bytes = fs::read(path).expect("Unabel to read The File");

    let mut data = bytes.iter();

    if file_type_check(&mut data){
        
    }
}