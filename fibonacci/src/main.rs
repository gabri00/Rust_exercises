use std::io;

fn main() {
    println!("Enter how many Fibonacci numbers to print:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let count: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid non-negative integer.");
            return;
        }
    };

    // Only print the sequence
    println!("First {} Fibonacci numbers:", count);
    print_fibonacci(count);

    // Also use an array to store the sequence
    let fib_seq = fibonacci_sequence(count);

    println!("First {} Fibonacci numbers:", count);
    for n in fib_seq {
        print!("{} ", n);
    }
    println!();
}

fn print_fibonacci(n: usize) {
    let mut a: u64 = 1;
    let mut b: u64 = 1;

    for _i in 0..n {
        print!("{} ", a);
        let next = a + b;
        a = b;
        b = next;
    }
    println!();
}

fn fibonacci_sequence(n: usize) -> Vec<u64> {
    let mut fib = vec![1; n];

    for i in 2..n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    fib
}

// Alternative solution using a preallocated capacity
// fn fibonacci_sequence(n: usize) -> Vec<u64> {
//     let mut fib = Vec::with_capacity(n);

//     for i in 0..n {
//         if i < 2 {
//             fib.push(1);
//         }
//         else {
//             let next = fib[i - 1] + fib[i - 2];
//             fib.push(next);
//         }   
//     }

//     fib
// }