use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]

struct Database{
    connection: HashMap<i32, String>
}

impl Database {
    fn new() -> Database {
        Database { connection: HashMap::new() }
    }

    fn connect(&mut self, name:String, id:i32){
        self.connection.insert(id, name);
    }
}

fn main() {
    let db = Mutex::new(Database::new());

    {
      
        let mut db_lock = db.lock().unwrap();
        db_lock.connect("Sql_server".to_owned(), 1245);
      
    }
    let main_db = db.lock().unwrap();

   print!("db: {main_db:?}");
}
