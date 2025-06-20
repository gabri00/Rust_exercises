# 🦀 Rust Learning Exercises

Welcome to this collection of beginner and intermediate exercises to help you learn and practice Rust!
These tasks will help you get comfortable with Rust's syntax, ownership system, standard library, and common idioms.

## 🟢 Beginner-Level Exercises

- [x] **Prime numbers**: Print all prime number in range 1 to *n*, where *n* is a user defined upper limit.
- [x] **FizzBuzz**: Print numbers from 1 to 100, replacing multiples of 3 with "*Fizz*", 5 with "*Buzz*", and both with "*FizzBuzz*".
- [x] **Palindrome Checker**: Ask the user for a string and check if it reads the same backward.
- [x] **Factorial Calculator**: Compute the factorial of a number using both *iterative* and *recursive* methods.
- [x] **Fibonacci Sequence**: Print the first *N* Fibonacci numbers.
- [x] **Guessing Game**: Build a number guessing game using `rand` and `io`.
- [x] **Sum of Digits**: Input a number and return the sum of its digits (e.g., `1234 → 1 + 2 + 3 + 4 = 10`).
- [x] **Armstrong Number Checker**: Check if a number is an Armstrong number (e.g., `153 → 1³ + 5³ + 3³ = 153`).

## 🟡 Intermediate-Level Exercises

- [x] **Prime Number Generator**: Use the Sieve of Eratosthenes algorithm to generate prime numbers up to a given limit.
- [ ] **Simple Calculator**: Parse and evaluate expressions like `"3 + 5"`.
- [ ] **Todo List CLI**: Add, view, and remove tasks using in-memory storage or files.
- [ ] **Sorting a Vector**: Implement a simple sort algorithm like bubble sort, then compare it with Rust’s `.sort()`.

## 📦 Bonus: Crate-Based Practice

Enhance your projects using popular Rust crates:

- [`rand`](https://crates.io/crates/rand): Random number generation
- [`chrono`](https://crates.io/crates/chrono): Date and time handling
- [`regex`](https://crates.io/crates/regex): Regular expressions
- [`serde_json`](https://crates.io/crates/serde_json): JSON serialization/deserialization

---

## 📚 Getting Started

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. Create a new project:
   ```bash
   cargo new my_project
   cd my_project
   ```
3. Write your solution in `src/main.rs`
4. Build and run:
   ```bash
   cargo build
   cargo run
   ```