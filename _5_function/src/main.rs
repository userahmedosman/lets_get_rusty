fn main() {
    let s: i32 = sum(25, 30);

    println!("Sum is: {}", s);
    message("Don't open this message");
}


fn sum(x: i32, y: i32) -> i32 { // return type function
    return x + y;
}

fn message(msg: &str){ //procedure type function
    println!("Message: {}", msg);
}