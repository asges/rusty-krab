use std::io;
use std::cmp::Ordering;
use rand::Rng; //trait that defines methods which random number generators implement

fn main() {
    println!("Welcome to 'Guess the Number'!");

    //Specify a number between 1 and 100 because as of 15/7/2020, std lib doesn't have a
    // random number generator yet. Library crate exists though
    let secret_number = rand::thread_rng().gen_range(1,101);
        println!("The secret number is : {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse()   
            .expect("Please type a number!");

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            }
        }
    }
}
