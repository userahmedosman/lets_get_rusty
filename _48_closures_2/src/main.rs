struct Credentials<T> where T: Fn(&str, &str) -> bool {
    username:String,
    password:String,
    validation: T
}

impl<T> Credentials<T> where T: Fn(&str, &str) -> bool {
    fn validate(&self) -> bool {
        (self.validation)(&self.username, &self.password)
    }
}

fn main() {

  
    let validate =  |username:&str, password:&str| {
        !username.is_empty() && 
        !password.is_empty() &&
        password.len() >= 8 &&
        password.contains(['@', '!', '#', '$', '%', '^', '&', '*'])
    };

   

    let cred = Credentials {
        username:"ahmed".to_owned(),
        password:"p@ssword".to_owned(),
        validation: validate
    };
    println!("Check credential is valid: {}", cred.validate());

    let password_validate = get_password_validator(8, true);
    let default = get_default_cred(password_validate);

    print!("Check credential is valid again: {}", default.validate())
}


fn get_default_cred<T>(f: T) -> Credentials<T> where T: Fn(&str, &str) -> bool {
    Credentials { 
        username: "guest".to_owned(), 
        password: "p@assword".to_owned(), 
        validation: f 
    }
}

fn get_password_validator(min_len: usize, special_char: bool) -> 
Box<dyn Fn(&str, &str) -> bool>{
    
    if special_char {
        Box::new(move |_:&str, password:&str | {
            password.len() >= min_len &&
            password.contains(['@', '!', '#', '$', '%', '^', '&', '*'])
        })
    }else {
        Box::new(move |_:&str, password:&str| !password.len() >= min_len)
    }
}