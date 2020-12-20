fn main() {
    // Collections: Tuples can have elements with different types - arrays all members must have
    // same type
    let tuple = (20, 'a');
    let array = [1,2,3];

    println!("Let's print this: {:?}", tuple);

    let a = [1, 2, 3, 4, 5];
    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);
}
