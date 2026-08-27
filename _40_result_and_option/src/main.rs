use std::{io, fs};
fn main() {
    let line = read_text("example.txt");
    
    if let Ok(Some(x)) =  line{
        print!("{}", x)
    }else if let Err(error) = line {
        panic!("Error: {}", error);
    }
    
}
fn read_text(filename: &str) -> Result<Option<String>, io::Error> {
   fs::read_to_string(filename).map(|s | {
        s.lines().last().map(|l | { 
            l.to_owned()
        })
    })
}