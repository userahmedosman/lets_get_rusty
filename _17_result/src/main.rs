
fn main() {
    let name = get_name(1);

    if let Some(n) = name {
        println!("\"name\":\"{}\"", n)
    }
    
}

fn get_name(id: i32) -> Option<String> {
    let query = format!("GET user FROM users WHERE id={id}");

    let name = db_query(query);
    
    name.ok()
   
}

fn db_query(s: String) -> Result<String, String> {
    if s.is_empty() {

        return Err(String::from("Query is malformed or is empty"));
    }

    Ok(String::from("Hajo"))
}