use std::io;

fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }

    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=((limit as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i*i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(index, &prime)| if prime { Some(index) } else { None }) // .filter(|(_, &prime)| prime).map(|(index, _)| index)
        .collect()
}

fn main() {
    println!("Prime Number Generator (Sieve of Eratosthenes)");
    println!("Enter the upper limit:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let limit: usize = input.trim().parse().expect("Please enter a valid number");

    let primes = sieve_of_eratosthenes(limit);

    println!("Prime numbers up to {}:", limit);
    println!("{:?}", primes);
}
