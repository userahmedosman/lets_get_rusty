
use std::ops::{Deref, DerefMut};

struct CustomSmartPT<T> {
    value: T
}

impl<T> CustomSmartPT<T> {
    fn new(value: T) -> CustomSmartPT<T> {
        CustomSmartPT { value }
    }
}

impl<T> Deref for CustomSmartPT<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> DerefMut for CustomSmartPT<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

fn main() {
    let s = CustomSmartPT::new(Box::new("Lets get Rusty".to_owned()));
   
    print(&s);  // Deref Coercion = &CustomSmartPT<T> -> &Box<T> -> &String -> &str


}

fn print(s: &str) {
        println!("{}", s);
}
