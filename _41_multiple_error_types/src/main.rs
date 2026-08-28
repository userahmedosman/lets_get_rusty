use std::{fs, io, num::ParseIntError};

fn main() {
    let content = parse_file_text_to_int("example.txt");
    match content {
        Ok(i) => {println!("parsed result: {}", i);}
        Err(e) => {
            match e {
                ParseError::File(e) => {
                    panic!("error: {}", e);
            },
                ParseError::Parse(e) => {
                    panic!("error: parse failed -> {}", e)
            }
        }
    }
    }
}

enum ParseError{
    File(io::Error),
    Parse(ParseIntError)
}


fn parse_file_text_to_int(filename: &str) -> Result<i32, ParseError>{
    let content = fs::read_to_string(filename)
                .map_err(|e:io::Error | ParseError::File(e))?;

    let parsed = content.parse()
                .map_err(|e:ParseIntError | ParseError::Parse(e))?;

    Ok(parsed)
}