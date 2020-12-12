use std::io;

fn factorial(num: u64) -> u64 {
    if num <= 1 {
        return 1;
    }

    num * factorial(num - 1)
}

fn main() {
    println!("Input your number: ");

    let mut input_string = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    let input_num = input_string
        .trim()
        .parse::<u64>()
        .expect("That's not a number!");

    println!("The factorial of your number is {}", factorial(input_num));
}
