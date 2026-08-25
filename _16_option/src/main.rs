fn main() {
    let name = get_name(1);

    if let Some(n) = name {
        println!("\"name\":\"{}\"", n)
    }
    
}

fn get_name(id: i32) -> Option<String> {
    let name = String::from("Hajo");
    if id == 1 {

        
      return Some(name);
    }
    None
}