
pub fn login(cred: model::Credential){
      println!("Login Successful !");
      crate::database::get_user_data();
    }

    fn logout() {
        println!("User loged out");
    }

pub mod model;