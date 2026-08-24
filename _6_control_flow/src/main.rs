fn main() {
    let x: i32 = 5;

    if(x > 10){
        println!("Greater than 10");
    }else if (x < 10){
        println!("Smaller than 10");
    }else{
        println!("This number is insane");
    }

    let y: i32 = if x > 5 { 1 } else { -1 };
    println!("Y is : {}", y);

    looper();
    while_loop();
    for_loop();
}

fn looper(){
    let mut x: i8 = 0;
   'outer:  loop{  // labled loop
        println!("loop on until x reach 10: x now-> {}", x);
        x+=1;
        if x > 10 {break 'outer;}
    }

    let lp: i8 = loop {
        break 5;
    };

    println!("Loop result: {}", lp);
}

fn while_loop(){
    let mut x: i8 = 20;
    while (x < 30){
        println!("While looping: {}", x);
        if x > 30 {break;}
        x+=1;
    }
}


fn for_loop(){
    let arr = [20, 30, 40, 50, 60];

    for ele in arr{
        println!("{}", ele);
    }
}