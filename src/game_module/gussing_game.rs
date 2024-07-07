use std::{cmp::Ordering, io};
use rand::Rng;

pub fn fn_gussing_game(){
    println!("*******Hello Welcome to Gussing Games!*********");
    //generate random number between 1 - 100 inclusive
    //rand::thread_rng function that gives us the particular random number generator we’re going to use
    //gen_range method on the random number generator. It is defined by the Rng trait 
    //gen_range method takes a range expression as an argument and generates a random number in the rang
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess Number and Please input your guess");
        // guess is mutable variable and initially empty instance of String is assigned
        let mut guess = String::new();
        
        io::stdin()
            .read_line(&mut guess) // get user input
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a Number");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                println!("The secret number is: {secret_number}");
                break;
            }            
        }
    }    
}