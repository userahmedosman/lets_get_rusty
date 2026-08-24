fn main(){
    let b : bool = true; //boolean

    // unsigned integer
    let uint: u8 = 1; // lowest bit
    let uini: u128 = 1; // higest bit
     
    //signed int
    let sint: i8 = -1;
    let sint: i128 = 1;

    // float numbers

    let f1: f32 = 1.0;
    let f2: f64 = 1.10;

    // platform specific integers
    let p1: usize = 1;
    let p2: isize = 1;

    // characters, &str, and String
    let c: char = 'a';
    let s1: &str = "this is string";
    let s2: String = String::from("This is string");

    //arrays
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    let le: i32 = arr[2];
    println!("Value of le: {le}");

    //Tubles
    let t1 = (1, 2, 3);
    let t2 = (1, 'c', "this is str");
    let ts = t2.2;

    let (p1, c, s1) = t2;
    

    let unit: () = ();

    // Type aliasing
    type age = u8;

    let x: age = 25;
}