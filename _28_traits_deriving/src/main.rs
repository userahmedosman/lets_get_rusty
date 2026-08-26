#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: f32
}


fn main() {
    let p1 = Point{x:3, y:1.0};
    let p2 = Point{x:3, y:1.0};
    let p3 = Point{x:5, y:5.0};

    print!("{:?}", p1);
    print!("{:?}", p1 == p2);
    print!("{:?}", p1 == p3);
}
