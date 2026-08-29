use std::{thread, sync::mpsc};

fn main() {

    let (tx, rx) = mpsc::channel();
  
    let sentences = [
        "My dream is not to live like a dead".to_owned(),
        "Rust is funny, but have little challenging concepts".to_owned(),
        "Why i can't be me".to_owned(),
        "I will live like i wish".to_owned()
    ];

    for se in sentences{
        let tx_clone = tx.clone();
        thread::spawn(move | | {
            let reversed:String = se.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx); // drop the transmiter channel once the loop ends
    for r in rx{

        print!("\n{r}");
    }
}
