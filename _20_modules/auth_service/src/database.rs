pub enum Status {
        Connected,
        Interupted
    }

    pub fn database_connection() -> Status {
        return Status::Connected;
    }

    pub fn get_user_data(){
        println!("User data:");
        println!("Registration Date: 20-11-2024");
        println!("Account status: Active");
        println!("Location: London, Chelsea hills 2141");
    }