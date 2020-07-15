use std::io;

fn main() {
    println!("Welcome to 'Guess the Number'!");

    println!("Please input a number.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);

    //Generate a secret number to compare to

    //Comparing the secret number to the guessed Number

    //Allowing for multiple guesses with looping

    //Exiting after correct guess
}
