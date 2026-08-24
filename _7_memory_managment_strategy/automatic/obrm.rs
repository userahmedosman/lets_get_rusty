
struct Car {};

fn function_that_can_panic(){
    println!("this function may fail");
}

fn check_data() -> bool {
    return true;
}

fn memory_example(){
    let car = Box::new(Car {});
    let car2 = car; // implicit ownership move

    let carshared = Rc::new(Car {}); // Reference count shared ownership
    let carshared2 = carshared.clone(); // ownershiop clone.
    let my_string = String::from("LGR");
    function_that_can_panic();
    if(!check_data()) return;
}

fn file_example(){
    let path = Path::new("example.txt");
    let file = File::open(&path).unwrap();
    function_that_can_panic();
    if(!check_data()) return;
}