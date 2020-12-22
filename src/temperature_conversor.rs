use std::io;

fn main() {
    println!("Welcome to the temperature conversor program");
    println!("If you want to convert from Celcius to Fahrenheit, press C - If you want to convert from Fahrenheit to Celcius, press F");

    let mut option = String::new();

    io::stdin()
        .read_line(&mut option);

    let trimmed_option = option.trim();

    if trimmed_option == "C" {
        println!("Celcius to Fahrenheit")
    }
    else if trimmed_option == "F" {
        println!("Fahrenheit to Celcius")
    }
    else {
        println!("Wrong choise")
    }
}
