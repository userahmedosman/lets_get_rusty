use _64_declarative_macros_2::*;
use std::collections::HashMap;
fn main() {
let mut hm = hashmap!(String, i32);
hm.insert("banana".to_owned(), 20);
hm.insert("apple".to_owned(), 10);
hm.insert("orange".to_owned(), 6);
hm.insert("mango".to_owned(), 15);
    
for (key, value) in &hm {
    println!("{}: {}", key, value);
}

print!("\ncustom direct initilization of hashmap: \n");
    
    let hm2 = hashmap!(
        "kiwi".to_owned() => 8,
        "pinapple".to_owned() => 12,
        "grape".to_owned() => 18,
        "strawberry".to_owned() => 16
    );
    
    for (key, value) in &hm2 {
        println!("{}: {}", key, value);
    }
}
