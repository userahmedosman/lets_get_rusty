use std::{fs::File, io::Write};

fn main() {
    let file = File::open("file.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("failed to open file: {:?}", error);
        }
    };

   
}
