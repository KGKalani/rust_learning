// To practice with the concepts discussed in this chapter, try building programs to do the following:

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.
// When you’re ready to move on, we’ll talk about a concept in Rust that doesn’t commonly exist in other programming languages: ownership.

use std::io;

pub fn demo_common_concept_assignments(){
    println!("Common Concepts : \n\t1. Variables and Mutability \n\t2. Data Types \n\t3.Functions\n\t4. Control Flow");
    convert_fahrenheit_to_celsius();
}

fn convert_fahrenheit_to_celsius(){
    println!("Fahrenheit to Celsius (°F to °C) conversion calculator");
    println!("Enter fahrenhit amout to convert: ");

    let mut fahrenheit_amount = String::new();
    io::stdin().read_line(&mut fahrenheit_amount).expect("Not an number");

    let fahrenheit_amount = match fahrenheit_amount.trim().parse::<u32>(){
        Ok(num) => num,
        Err(_)=> {
            println!("Not a number");
            0
        }
    };
    
    let celsius_amount = (fahrenheit_amount - 32) as f32 / 1.8;
    println!("Fahrenheit : {fahrenheit_amount} F");
    println!("Celsius : {celsius_amount} C");
       
}