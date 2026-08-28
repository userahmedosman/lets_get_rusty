
trait Iterator{
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Str {}

impl Iterator for Str{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
       None
    }
}
fn main() {
    let mut mystr = Str {};
    let next = mystr.next();
}
