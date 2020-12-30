struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    // A way of destructuring. The names of the fields must match the names of the fields in the
    // struct
    let Point { x, y } = p;
    println!("{}", x);
    println!("{}", y);
    let Point { x: alter_x, y: alter_y } = p;
    println!("{}", alter_x);
    println!("{}", alter_y);
}
