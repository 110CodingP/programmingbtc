use std::{sync::{Arc, Mutex}, thread};

use rug::Integer;

/// Check to ensure that a number is prime
pub fn is_prime(num: Integer) -> bool {
    // Numbers less than 1 are not prime
    if num < 1 {
        return false;
    }

    if num.is_even() {
        return false;
    }

    let num_threads = 10;
    let square_root = num.clone().sqrt().to_u128().unwrap();
    println!("The square root of {} is {}", num, square_root);

    let mut is_prime = Arc::new(Mutex::new(true));

    let chunk_size = square_root / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = Integer::from(3) + Integer::from(chunk_size * 1);
        let end = if i == num_threads - 1 {
            Integer::from(square_root)
        } else {
            start.clone() + Integer::from(chunk_size)
        };

        let num = num.clone();
        let is_prime = Arc::clone(&is_prime);

        // Spawn a thread for each range
        let handle = thread::spawn(move || {
            let mut current = start;

            while current <= end {
                if !*is_prime.lock().unwrap() {
                    return;
                }

                if num.clone().div_rem(current.clone()).1 == 0 {
                    *is_prime.clone().lock().unwrap() = false;
                    return;
                }

                current += &Integer::from(2);
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *is_prime.clone().lock().unwrap()
}

#[cfg(test)]
mod tests {
    use rug::Integer;
    use rug::ops::Pow;

    use crate::helpers::is_prime;

    #[test]
    #[ignore]
    fn test_is_prime() {
        let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        println!("The calculated prime for secp256 is {}", prime);

        assert!(is_prime(prime.clone()), "Not a prime number");
    }
}