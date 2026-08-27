use std::rc::Rc;
struct Database {
    connection: String
}

struct Authentication {
    db: Rc<Database>
}

struct ContentManagment {
    db:Rc<Database>
}

fn main() {
    let database = Rc::new(Database {connection:"sql@/con:true".to_owned()});

    let auth = Authentication {db: Rc::clone(&database)};

    let content_managment = ContentManagment {db: Rc::clone(&database)};

    print!("\ndatabase connection: {}", database.connection);
    print!("\nauthentication db: {}", auth.db.connection);
    print!("\ncontent_managment db: {}", content_managment.db.connection);
}
