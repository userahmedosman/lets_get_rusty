mod auth_utils;
mod database;

pub use auth_utils::model::Credential;
use crate::database::Status;
pub fn authenticate(cred: Credential){
    if let Status::Connected = database::database_connection(){
        auth_utils::login(cred);
    }
}

