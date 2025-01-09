use std::{fmt::Debug, ops::{Add, Div, Mul, Rem, Sub}};
use rug::{integer::IsPrime, ops::{Pow, RemRounding}};

use helpers::is_prime;
use rug::{Integer, Assign};

pub mod helpers;

/// A FieldElement is a representation of an element in a finite field.
#[derive(Clone)]
pub struct FieldElement {
    num: Integer,
    prime: Integer,
}

impl FieldElement {
    /// Creates a new field element 
    pub fn new(num: Integer, prime: Integer) -> FieldElement {
        // assert!(is_prime(prime.clone()), "Number must be prime");
        assert_ne!(prime.is_probably_prime(30), IsPrime::No, "Number must be prime");

        FieldElement { num: num % prime.clone(), prime }
    }

    pub fn num(&self) -> Integer {
        self.num.clone()
    }

    pub fn order(&self) -> Integer {
        self.prime.clone()
    }

    pub fn pow(&self, exponent: Integer) -> Result<FieldElement, ()> {
        let expo = if exponent > 0 {
            exponent
        } else { 
            exponent + (self.prime.clone() - Integer::from(1))
        };
        
        match self.num.clone()
            .pow_mod(&expo, &self.prime) 
            {
                Ok(num) => Ok(FieldElement::new(num.rem_euc(self.prime.clone()), self.prime.clone())),
                Err(_) => Err(())
            }
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

        let num = (self.num + other.num).rem_euc(self.prime.clone());
        FieldElement::new(num, self.prime)
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let num = (self.num - other.num).rem_euc(self.prime.clone());
        println!("Num: {}", (15 % 31));
        FieldElement::new(num, self.prime)
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let num = (self.num * other.num).rem_euc(self.prime.clone());
        FieldElement::new(num, self.prime)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        assert_eq!(self.prime, other.prime, "Primes must be equal");

        let divisor = other.pow(other.prime.clone() - Integer::from(2)).unwrap();

        let num = (self.num * divisor.num).rem_euc(self.prime.clone());
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
        let mut num = Integer::new();
        num.assign(3);
        let _ = FieldElement::new(num, Integer::from(20));
    }
    
    #[test]
    fn test_inequality() {
        let mut num = Integer::new();
        num.assign(2);
        let a = FieldElement::new(num.clone(), Integer::from(31));

        num.assign(2);
        let b = FieldElement::new(num.clone(), Integer::from(31));

        num.assign(19);
        let c = FieldElement::new(num, Integer::from(31));

        assert!(a.is_equal(&b), "Field elements should be equal");
        assert!(a == b, "Field elements should be equal");
        assert!(a.is_not_equal(&c), "Field elements should not be equal");
        assert!(b != c, "Field elements should not be equal");
    }

    #[test]
    fn test_addition() {
        let a = FieldElement::new(Integer::from(2), Integer::from(31));
        let b = FieldElement::new(Integer::from(15), Integer::from(31));
        let c = FieldElement::new(
            Integer::from(17), 
            Integer::from(31)
        );

        assert_eq!(a + b, c, "Addition failed");

        let a = FieldElement::new(
            Integer::from(17), 
            Integer::from(31)
        );
        let b = FieldElement::new(Integer::from(21), Integer::from(29));
        let result = panic::catch_unwind(|| a + b);
        assert!(result.is_err(), "Primes must be equal");
    }

    #[test]
    fn test_subtraction() {
        let a = FieldElement::new(
            Integer::from(29), 
            Integer::from(31)
        );
        let b = FieldElement::new(
            Integer::from(4), 
            Integer::from(31)
        );
        assert_eq!(a - b, FieldElement::new(
            Integer::from(25),
            Integer::from(31)
            )
        );

        let a = FieldElement::new(
            Integer::from(15), 
            Integer::from(31)
        );
        let b = FieldElement::new(
            Integer::from(30), 
            Integer::from(31)
        );
        assert_eq!(a - b, FieldElement::new(
            Integer::from(16),
            Integer::from(31)
        ))
    }

    #[test]
    fn test_multiply() {
        let a= FieldElement::new(Integer::from(24), Integer::from(31));
        let b = FieldElement::new(Integer::from(19), Integer::from(31));
        assert_eq!(
            a * b, 
            FieldElement::new(Integer::from(22), Integer::from(31))
        );
    }

    #[test]
    fn test_pow() {
        let a = FieldElement::new(Integer::from(17), Integer::from(31));
        assert_eq!(a.pow(Integer::from(3)), Ok(FieldElement::new(Integer::from(15), Integer::from(31))));

        let a = FieldElement::new(Integer::from(5), Integer::from(31));
        let b = FieldElement::new(Integer::from(18), Integer::from(31));
        assert_eq!(a.pow(Integer::from(5)).unwrap() * b, FieldElement::new(Integer::from(16), Integer::from(31)));
    }

    #[test]
    fn test_divison() {
        let a = FieldElement::new(Integer::from(3), Integer::from(31));
        let b = FieldElement::new(Integer::from(24), Integer::from(31));
        assert_eq!(a / b, FieldElement::new(Integer::from(4), Integer::from(31)));

        let a = FieldElement::new(Integer::from(17), Integer::from(31));
        assert_eq!(a.pow(Integer::from(-3)), Ok(FieldElement::new(Integer::from(29), Integer::from(31))));

        let a = FieldElement::new(Integer::from(4), Integer::from(31));
        let b = FieldElement::new(Integer::from(11), Integer::from(31));
        assert_eq!(a.pow(Integer::from(-4)).unwrap() * b, FieldElement::new(Integer::from(13), Integer::from(31)));
    }
}
