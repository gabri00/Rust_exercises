use std::io;

fn main() {
    // Get user input
    println!("Enter the upper limit for FizzBuzz:");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let upper_bound: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    for i in 1..=upper_bound {
        if i % 3 == 0 && i % 5 == 0 {
            print!("FizzBuzz, ");
        }
        else if i % 3 == 0 {
            print!("Fizz, ");
        }
        else if i % 5 == 0 {
            print!("Buzz, ");
        }
        else {
            print!("{}, ", i);
        }
    }
}
