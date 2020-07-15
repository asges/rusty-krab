use std::io;
use rand::Rng; //trait that defines methods which random number generators implement

fn main() {
    println!("Welcome to 'Guess the Number'!");

    //Specify a number between 1 and 100 because as of 15/7/2020, std lib doesn't have a
    // random number generator yet. Library crate exists though
    let secret_number = rand::thread_rng().gen_range(1,101);

    println!("The secret number is : {}", secret_number);
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    println!("You guessed: {}", guess);

    //Generate a secret number to compare to


    //Comparing the secret number to the guessed Number

    //Allowing for multiple guesses with looping

    //Exiting after correct guess
}
