fn main() {
    let mut s1 = String::from("Linux");
    let r1 = &s1; // borrwing by reference

    borrow_string(r1);
    let r2 = &mut s1; // make it mutable when referencing where mutation can happen.
    concatinate_string(r2);

    let s2 = generate_string();
    println!("I hate {}", s2);
    
}

fn concatinate_string(st: &mut String){
    st.push_str(" is my favorite os");
}

fn borrow_string(st: &String){
    println!("My OS is {}",st);
}

fn generate_string() -> String {
    String::from("Windows")
}