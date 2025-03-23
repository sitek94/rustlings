use rand::Rng;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=2);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    if guess == secret_number {
        println!("Correct: {guess} is a secret number!");
    } else {
        println!("Wrong: Secret number is {secret_number}!");
    }
}
