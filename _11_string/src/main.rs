
fn main() {
    // strings in rust are UTF-8
    let s1 = "ሰለመተ።";
    let s2 = String::from("ሰለመተ።"); // 1 char is 3 bit
    let s3 = s2.to_string();
    let s4 = s2.to_owned();
    println!("{}", s4);
    let s5 = &s1[..6];

    println!("this is geez: {}", s5);

    string_manuplation();
    string_concat();
    string_formating();
    string_iteration_by_bits();
}


fn string_manuplation(){
    let mut s = String::from("foo");
    s.push_str("baz");
    println!("{}", s);
    s.replace_range(.., "baz");
    println!("{}", s);

}

fn string_concat(){
    let s1 = String::from("hello ");
    let s2 = String::from(" world");

    let s3 = s1 + &s2;
    println!("{}", s3);

    let s4 = ["ping ", "pong"].concat();
    println!("{}", s4);
    let s5 = format!("{}{}", "first ", "second");
    println!("{}", s5);
    let s6 = concat!("tiki ", "taka");
    println!("{}", s6);
}

fn string_formating(){
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s4);
}

fn string_iteration_by_bits(){
    println!("iteration by bit");
    for b in "ሰለመተ።".bytes(){
        println!("{}", b);
    }
    println!("iteration by char");
    for c in "ሰለመተ።".chars(){
        println!("{}", c);
    }

   
}