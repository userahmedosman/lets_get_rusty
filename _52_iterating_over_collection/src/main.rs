use std::collections::HashMap;

fn main() {

    let mut teams:HashMap<String, i32> = HashMap::new();
    teams.insert("blue team".to_owned(), 56);
    teams.insert("red team".to_owned(), 62);
    teams.insert("green team".to_owned(), 47);
    teams.insert("yellow team".to_owned(), 77);

    
    for (team, point) in teams{

        println!("{team} Got: {point}");
    }
}
