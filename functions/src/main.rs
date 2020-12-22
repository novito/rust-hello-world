fn main() {
    println!("Hello, world!");
    println!("{}", another_function(4));
}

fn another_function(x: i32) -> i32 {
    let test = [1,2,3,4];

    for i in test.iter() {
        println!("{}", i);
    }

    for j in (1..4).rev() {
        println!("{}", j);
    }

    x + 4
}
