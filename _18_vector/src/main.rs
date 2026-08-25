fn main() {
    // first way of initializing vectors

    let mut v: Vec<String> = Vec::new();
    v.push(String::from("one"));
    v.push(String::from("two"));
    v.push(String::from("three"));
    
    // v.remove(0) // to remove any element by index
    let a = v.get(0); // to get any element by index and returns Option<T>
    let b = &v[0]; // to get any element by index but it can panic if not found 
    
    if let Some(x) = a {
    println!("{}", x);
    }

    for s in &mut v {
        s.push_str("!");
    }

    for x in &v {
        println!("{}", x);
    }
    
    // second way of initializing vector using macro

    let v2 = vec![1, 2, 3 ,4];


    println!("Hello, world!");
}
