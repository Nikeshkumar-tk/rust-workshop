use std::io;
use rand::Rng;
fn main() {
    println!("Enter a Number");
    let mut num_val = String::new();
    io::stdin().read_line(&mut num_val ).expect("Something happened while recieving user input");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}", secret_number);
    println!("You guessed the number:{num_val}")
}
