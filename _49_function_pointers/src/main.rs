fn main() {
    let ten = 10; // if a clouser captures external variable it can't get coehred with fn
   
    let c2 = |y: &i32 | *y < 20;

    let result = check_both_clouser_bool(c1, c2, &15);

    
    println!("Result: {}", result);
}

fn c1(d: &i32) -> bool{ // function can be coerced to clousers viceversa

 *d > 10
}

fn check_both_clouser_bool<X, Y, V>(c1: X, c2: Y, value: &V) -> bool
 where X: Fn(&V) -> bool, Y: Fn(&V) -> bool
 {

    c1(value) && c2(value)
}