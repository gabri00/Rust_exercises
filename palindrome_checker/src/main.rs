use std::io;

// Ignore case and non-alphanumeric characters.
fn is_palindrome(text: &str) -> bool {
    let cleaned_text: String = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let reversed_text: String = cleaned_text.chars().rev().collect();

    cleaned_text == reversed_text
}

fn main() {
    println!("Enter a word or phrase to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    if is_palindrome(&input) {
        println!("It's a palindrome!");
    }
    else {
        println!("Not a palindrome.");
    }
}
