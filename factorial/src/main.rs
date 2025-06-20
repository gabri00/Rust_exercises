use std::io;
use num_bigint::BigUint;
use num_traits::One;

fn main() {
    println!("Enter a non-negative integer to compute its factorial:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid non-negative integer.");
            return;
        }
    };

    println!("Choose method: (1) Recursive  (2) Iterative");

    let mut method_input = String::new();
    io::stdin().read_line(&mut method_input).expect("Failed to read method");
    let method = method_input.trim();

    let result = match method {
        "1" => factorial_recursive(n),
        "2" => factorial_iterative(n),
        _ => {
            println!("Invalid method. Choose 1 or 2.");
            return;
        }
    };

    println!("Factorial of {} is:\t{}", n, result);
}

// Recursive factorial (uses u64 for small numbers, BigUint for large)
fn factorial_recursive(n: u64) -> String {
    if n < 21 {
        fn recurse(n: u64) -> u64 {
            if n <= 1 { 1 }
            else { n * recurse(n - 1) }
        }
        
        recurse(n).to_string()
    }
    else {
        fn recurse_big(n: u64) -> BigUint {
            if n <= 1 {
                BigUint::one()
            } else {
                BigUint::from(n) * recurse_big(n - 1)
            }
        }
        
        recurse_big(n).to_string()
    }
}

// Iterative factorial (uses u64 for small numbers, BigUint for large)
fn factorial_iterative(n: u64) -> String {
    if n < 21 {
        (1..=n).product::<u64>().to_string()
    }
    else {
        let mut result = BigUint::one();
        for i in 1..=n {
            result *= i;
        }
        result.to_string()
    }
}
