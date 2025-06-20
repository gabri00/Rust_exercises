use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("I'm thinking of a number between 1 and 100...");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read input");

        // Shadowing the variable guess to avoid using two variables
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win! The number was {}.", secret_number);
                break;
            }
        }
    }
}
