use std::{collections::HashMap, thread, sync::{Arc, Mutex}};

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
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];
    for i in 1..10{
        let db = Arc::clone(&db);

        let handle = thread::spawn(move || {
             let mut db_lock = db.lock().unwrap();
             db_lock.connect("Sql_server_".to_owned() + &i.to_string(), i);
        });

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    let main_db = db.lock().unwrap();

   print!("db: {main_db:?}");
}