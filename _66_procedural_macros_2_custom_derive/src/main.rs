use _66_procedural_macros_2_custom_derive::*;
use syn::Data;


trait Log{
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
}

#[derive(Debug, Log)]
struct Database {
    url: String,
    connection_pool_size: u32,
}

impl Database {
    fn new(url: String) -> Self {
        Database { url, connection_pool_size:0 }
    }
    fn connect(&mut self) {
        self.connection_pool_size += 1;
        self.info(format!("new connection added to {}", self.url).as_str());

        if self.connection_pool_size > 50 {
            self.warn(format!("connection pool size is too large: {}", self.connection_pool_size).as_str());
        }
    }
}
fn main() {
    let mut db = Database::new("localhost:5432".to_string());
    for _ in 0..55 {
        db.connect();
    }
}
