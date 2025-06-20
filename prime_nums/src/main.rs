use std::io;

fn is_prime(n: u32) -> bool {
    if n <= 3 {
        return n > 1;
    }

    if n % 2 == 0 {
        return false;
    }

    let sqrt_n = (n as f64).sqrt() as u32;
    for i in (3..=sqrt_n).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}


fn main() {
    println!("Enter an upper bound (e.g., 7): ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let upper_bound: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };

    println!("Prime numbers from 1 to {}:", upper_bound);
    for num in 1..=upper_bound {
        if is_prime(num) {
            print!("{}, ", num);
        }
    }

    println!();
}
