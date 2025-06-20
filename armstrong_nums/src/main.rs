use std::io;

fn main() {
    println!("Enter a positive value:");

    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read input");

    let number = number.trim();
    if !number.chars().all(|c| c.is_digit(10)) {
        println!("Invalid input. Please enter digits only.");
        return;
    }
    
    let digits = number.to_string()
    .chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect();

    let number = number.parse().unwrap();

    if is_armstrong(number, digits) {
        println!("{} is an Armstrong number", number);
    }
    else {
        println!("{} is NOT an Armstrong number", number);
    }

}

fn is_armstrong(n: u32, d: Vec<u32>) -> bool {
    n == d.iter().map(|i| i.pow(d.len() as u32)).sum()
}
