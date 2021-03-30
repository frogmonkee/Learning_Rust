// uses crate(s) defined as dependencies in cargo.toml
extern crate rand;

// uses input/output (io) library to obtain user input and print the result
use std::io;
use rand::Rng;
use std::cmp::Ordering; 

fn main() {
    loop {
        println!("Guess the number!");    
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            // Will return argument if Result is Err. Will return guess if Result is OK
            .expect("Failed to read line.");
        
        //converts string to int. Trim methods removes whitespace and parse method parses string to a number
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
            // Will return argument if Err. Eg. emoji inputted as argument for guess
            

        println!("You guessed: {}", guess);

        // Generates a secret number using a local RNG (thread_rng) and create a range of 1 to 100
        let secret_number = rand::thread_rng().gen_range(1, 10);
        println!("The secret number is: {}", secret_number);    

        //Compares secret_number to guess using cmp module and Ordering (enum) to compare values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}