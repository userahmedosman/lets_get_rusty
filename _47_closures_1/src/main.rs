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
}
