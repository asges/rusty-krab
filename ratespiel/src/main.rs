use std::io;

fn main() {
    println!("Welcome to 'Guess the Number'!");

    println!("Please input a number.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
