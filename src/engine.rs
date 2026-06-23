pub mod asm;
pub mod dis;
pub mod isa;
pub mod vm;

pub mod helpers {
    pub fn file_type_check<'a, I>(data: &mut I) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        let mut type_of_file = Vec::<u8>::new();
        let mut c = 4;
        while c != 0
            && let Some(i) = data.next()
        {
            type_of_file.push(*i);
            c -= 1;
        }

        if type_of_file.len() == 4 {
            if type_of_file[0] == 77
                && type_of_file[1] == 86
                && type_of_file[2] == 77
                && type_of_file[3] == 00
            {
                return true;
            }
        }

        false
    }

    pub fn check_version<'a, I>(data: &mut I) -> bool
    where
        I: Iterator<Item = &'a u8>,
    {
        if let Some(v) = data.next() {
            if *v == 1 {
                return true;
            }
        }
        false
    }
}
