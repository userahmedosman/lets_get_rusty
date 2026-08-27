use std::rc::Rc;
use std::cell::RefCell;
struct Database {
    connection: String,
    max_con: u32
}

struct Authentication {
    db: Rc<RefCell<Database>>
}

struct ContentManagment {
    db:Rc<RefCell<Database>>
}

fn main() {
    let database = Rc::new(RefCell::new(Database {
        connection:"sql@/con:true".to_owned(),
        max_con: 100
    }));

    print!("\nconnection load: {}", database.borrow().max_con);

    let auth = Authentication {db: Rc::clone(&database)};

    let content_managment = ContentManagment {db: Rc::clone(&database)};
   database.borrow_mut().max_con = 200;
    //let mut lb2 = database.borrow_mut(); 
    // will not show compile time error but runtime time will panic due to already mutable RefCell mutably reborrowing attempt


    print!("\nupdated connection load: {}", database.borrow().max_con);
    
}
