fn main() {
    let s1 = String::from("hello");

    // Here we are NOT transfering ownership of s1 into print_string, so after the
    // function call we should still refer to s1
    // What we are doing here is passing a reference to s1, but the owner of s1 is still main
    // function
    //print_string(&s1);

    // This should throw an error
    println!("{}", *s1);
}

fn print_string(s1: &String) {
    println!("{}", *s1);
}
