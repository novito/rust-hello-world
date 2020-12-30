fn main() {
    let s1 = String::from("hello");

    // Here we are transfering ownership of s1 into print_string, so after the
    // function call we should not be able to refer to s1
    println!("{}", *s1)

    // This should throw an error
    // println!("{}", s1)
}
