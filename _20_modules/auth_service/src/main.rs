use rand::prelude::*;
use hajo_auth_service_demo::{Credential};

fn main(){
    let timeout = thread_rng().gen_range(100..500);

    println!("the time out is: {}", timeout);
    let cred = Credential {
        username: "hajoosman".to_owned(),
        password: "pass112233".to_owned()
    };

    hajo_auth_service_demo::authenticate(cred);
}