use rug::Integer;

/// Check to ensure that a number is prime
pub fn is_prime(num: Integer) -> bool {
    // Numbers less than 1 are not prime
    if num < 1 {
        return false;
    }

    // Numbers must not be divisible by any other number 
    // Check till the square root for faster computation
    for i in 2..num.clone().sqrt().to_u32().unwrap() {
        if num.clone() % i == 0 {
            return false;
        }
    }

    true
}