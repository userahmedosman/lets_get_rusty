use std::{fs::File, io::{self, Read}};

fn main() {
    let file_content = read_file("src/example.txt");
    let file_content = match file_content {
        Ok(contnet) => contnet,
        Err(error) => {
            panic!("can't open file: {}", error)
        }
    };

    println!("{file_content}");

    let user  = User {
        firstname:"Ahmed".to_owned(),
        secondname: "Osman".to_owned()
    };
     let short = user.get_user_short_name();

     if let Some(name) = short {
    print!("{}", name);
     }
}

fn read_file(filename: &str) -> Result<String,io::Error> {
    
    let mut content = String::new();
    File::open(filename)?.read_to_string(&mut content)?;
    Ok(content)

}

struct User  {
    firstname: String, 
    secondname: String
}

impl User {
    fn get_user_short_name(&self) -> Option<String> {
        let first = self.firstname.chars().next()?;
        let second = self.secondname.chars().next()?;

        let name = format!("{first}.{second}").to_owned();
        if name.is_empty(){
           return None
        }
        Some(name)
    }
}