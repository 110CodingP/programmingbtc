/// Check to ensure that a number is prime
pub fn is_prime(num: i64) -> bool {
    // Numbers less than 1 are not prime
    if num < 1 {
        return false;
    }

    // Numbers must not be divisible by any other number 
    // Check till the square root for faster computation
    for i in 2..(num as f64).sqrt() as i64 {
        if num % i == 0 {
            return false;
        }
    }

    true
}