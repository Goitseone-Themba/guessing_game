use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Guessing the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1.. 101);

    loop {
        println!("Please enter your guess.");
    
        let mut guess: String = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Error reading the guess, Enter a  number");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }  
        }
    }
}