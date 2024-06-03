use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1.. 101);

    println!("The secret number is {}", secret_number);

    println!("Please enter your guess.");

    let mut guess: String = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read guess");

    let guess: u32 = guess.trim().parse().expect("Failed to read integer.");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"), 
    }
}