use std::{thread, time::Duration};
fn main() {
  let handle =  thread::spawn(|| {
        for i in 0..20{

            println!("Spawned thread: {i}");
            thread::sleep(Duration::from_secs(1));
        }
    });

    for i in 0..15{
        println!("Main thread: {i}");
        thread::sleep(Duration::from_secs(2));
    }

    handle.join().unwrap();
}
