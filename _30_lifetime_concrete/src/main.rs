fn main() {
    let msg;
    {
        let msg1 = String::from("Lets get Rusty");
        msg = &msg1;
        // println!("{}", msg); will print normally 
    } // msg1 will end its life here, so msg is about to point to dangling reference outside of this scope
    
    //will show compile-time error, does not live long enough
    // println!("{}", msg); 

  {
    let mut st = String::from("Rust is fast");
    let r1 = &st;
    // if we exchange line 17 down to 18 vice-versa error will show
    //cannot borrow `st` as mutable because it is also borrowed as immutable
    print!("{}", r1); // will run normally st still owner of the data
    let r2 = &mut st;
    r2.push_str("r1:");
  }

}
