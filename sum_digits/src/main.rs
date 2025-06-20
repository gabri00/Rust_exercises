use std::io;

fn main() {
    println!("🔢 Sum of Digits Calculator");
    println!("Enter a positive integer:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let input = input.trim();

    // Ensure input contains only digits
    if !input.chars().all(|c| c.is_digit(10)) {
        println!("Please enter only digits (0–9).");
        return;
    }

    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    let sum: u32 = digits.iter().sum();

    let expression = digits
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<_>>()
        .join(" + ");

    println!("{} = {}", expression, sum);
}
