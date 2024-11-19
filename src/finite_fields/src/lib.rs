use std::{fmt::Debug, ops::{Add, Div, Mul, Rem, Sub}};

use helpers::is_prime;

mod helpers;

/// A FieldElement is a representation of an element in a finite field.
#[derive(Clone)]
pub struct FieldElement {
    num: i32,
    prime: i16,
}

impl FieldElement {
    /// Creates a new field element 
    pub fn new(num: i32, prime: i16) -> FieldElement {
        assert!(is_prime(prime.into()), "Number must be prime");

        FieldElement { num: num % prime as i32, prime }
    }

    pub fn num(&self) -> i32 {
        self.num
    }

    pub fn order(&self) -> i16 {
        self.prime
    }

    pub fn pow(&self, exponent: i32) -> FieldElement {
        let expo = if exponent > 0 {
            exponent
        } else { 
            exponent + (self.prime as i32 - 1)
        };
        
        let num = self.num.pow(expo as u32).rem_euclid(self.prime as i32);
        FieldElement::new(num, self.prime)
    }

    fn is_equal(&self, other: &FieldElement) -> bool {
        self == other
    }

    fn is_not_equal(&self, other: &FieldElement) -> bool {
        !self.is_equal(other)
    }
}

// Includes a formatter for the FieldElement struct
impl Debug for FieldElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

/// Implements the equality checker for FieldElement
/// Allows using operator `==` and `!=` to compare FieldElement instances
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
}

impl Eq for FieldElement {}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let num = (self.num + other.num).rem_euclid(self.prime as i32);
        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let num = (self.num - other.num).rem_euclid(self.prime as i32);
        println!("Num: {}", (15 % 31));
        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let num = (self.num * other.num).rem_euclid(self.prime as i32);
        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let divisor = other.pow(other.prime as i32 -2);
        println!("Divisor: {:?}", divisor);

        let num = (self.num * divisor.num).rem_euclid(self.prime as i32);
        FieldElement::new(num, self.prime)
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use super::*;

    #[test]
    #[should_panic(expected = "Number must be prime")]
    fn test_prime_must_prime() {
        let _ = FieldElement::new(3, 20);
    }
    
    #[test]
    fn test_inequality() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(2, 31);
        let c = FieldElement::new(19, 31);

        assert!(a.is_equal(&b), "Field elements should be equal");
        assert!(a == b, "Field elements should be equal");
        assert!(a.is_not_equal(&c), "Field elements should not be equal");
        assert!(b != c, "Field elements should not be equal");
    }

    #[test]
    fn test_addition() {
        let a = FieldElement::new(2, 31);
        let b = FieldElement::new(15, 31);
        let c = FieldElement::new(17, 31);

        assert_eq!(a + b, c, "Addition failed");

        let a = FieldElement::new(17, 31);
        let b = FieldElement::new(21, 29);
        let result = panic::catch_unwind(|| a + b);
        assert!(result.is_err(), "Primes must be equal");
    }

    #[test]
    fn test_subtraction() {
        let a = FieldElement::new(29, 31);
        let b = FieldElement::new(4, 31);
        assert_eq!(a - b, FieldElement::new(25, 31));

        let a = FieldElement::new(15, 31);
        let b = FieldElement::new(30, 31);
        assert_eq!(a - b, FieldElement::new(16, 31))
    }

    #[test]
    fn test_multiply() {
        let a= FieldElement::new(24, 31);
        let b = FieldElement::new(19, 31);
        assert_eq!(a * b, FieldElement::new(22, 31));
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(17,7);
        assert_eq!(a.pow(6), FieldElement::new(1, 7));

        let a = FieldElement::new(5, 31);
        let b = FieldElement::new(18, 31);
        assert_eq!(a.pow(5) * b, FieldElement::new(16, 31));
    }

    #[test]
    fn test_divison() {
        let a = FieldElement::new(3, 7);
        let b = FieldElement::new(24, 7);
        assert_eq!(a / b, FieldElement::new(1, 7));

        let a = FieldElement::new(17, 11);
        assert_eq!(a.pow(-3), FieldElement::new(8, 11));

        let a = FieldElement::new(4, 17);
        let b = FieldElement::new(11, 17);
        assert_eq!(a.pow(-4) * b, FieldElement::new(11, 17));
    }
}
