use std::thread;

fn main() {

    let name = String::from("Julian");

    thread::spawn( move | | {

        println!("{name}");
    });

   
}
