fn main(){
     // default constant 32bit variable
    let a: i32 = 5;
    println!("a value: {a}");
    // muteable variable
    let mut b: i32 = 6;
    println!("b value: {b}");
    b = 7;
    println!("b new value: {b}");

    // shadowng
    let _d:i32 = 25;
    let d:i32 = 30;
    println!("shadowed d value: {d}");


    // scope
    {
        let e = 20;
        println!("value of inner e: {e}");
    }
   let u_i: u8 = 10;
   println!("value of unsigned int u_I: {u_i}")
}