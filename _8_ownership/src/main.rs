fn main() {
    let s = String::from("Rust"); 
    let s1 = s; // now s1 variable is the owner of the value, and s is dead
    println!("Language name: {}", s1);
    print_string(s1.clone()); // now to share ownership we are cloning value instead of handling ownership

    let s2 = generate_string(); // ownership shiped out to this variable
    println!("Second language: {}", s2);

    let s3 = concatenate_string(s2); // ownership of s2 has moved to parameter function concatinate_string
    println!("{s3}");
}

fn concatenate_string(mut st: String) -> String {
    st.push_str(" is awesome!");
    return st;
}

fn print_string(st: String){
    println!("{st}");
}

fn generate_string() -> String {
    return String::from("C++");
}
