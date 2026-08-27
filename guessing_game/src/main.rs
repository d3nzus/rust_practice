use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    
    println!("Guess the number!");

    let correct = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is {correct}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // this removes whitespace from the string(trim) and parses it into an unsigned 32-bit integer
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    match guess.cmp(&correct) {
        Ordering::Less => println!("Too small!"), Ordering::Greater => println!("Too big!"), Ordering::Equal => println!("You win!")
    }
}