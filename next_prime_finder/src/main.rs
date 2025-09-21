use std::env;
use num_bigint::BigUint;
use next_prime_finder::{is_prime, next_prime};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <number>", args[0]);
        std::process::exit(1);
    }

    let num_str = &args[1];
    match num_str.parse::<BigUint>() {
        Ok(num) => {
            if is_prime(&num) {
                println!("{} is prime", num);
            } else {
                let next = next_prime(&num);
                println!("{} is not prime, next prime is {}", num, next);
            }
        }
        Err(_) => {
            eprintln!("Error: '{}' is not a valid non-negative integer.", num_str);
            std::process::exit(1);
        }
    }
}
