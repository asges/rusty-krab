use std::io;

fn main() {
    println!("Welcome to 'Guess the Number'!");

    //Specify a number between 1 and 100 because as of 15/7/2020, std lib doesn't have a
    // random number generator yet. Library crate exists though
    println!("Please input a number between 1 and 100.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);

    //Generate a secret number to compare to


    //Comparing the secret number to the guessed Number

    //Allowing for multiple guesses with looping

    //Exiting after correct guess
}
