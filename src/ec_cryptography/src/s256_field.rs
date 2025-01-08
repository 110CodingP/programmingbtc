use finite_fields::FieldElement;
use rug::integer::Order;
use rug::{Integer, Complete};
use rug::ops::Pow;
use std::fmt::{Debug, Formatter};

use crate::EllipticCurve;

pub struct S256Field {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl S256Field {
    pub fn new() -> S256Field {
        S256Field { x: None, y: None }
    }

    pub fn verify(&self, z: Integer, signature: Signature) -> bool {
        let s = FieldElement::new(signature.s.clone(), Self::order());
        let z = FieldElement::new(z, Self::order());
        let r = FieldElement::new(signature.r.clone(), Self::order());

        let u = z / s.clone();
        let v = r.clone() / s;

        let generator = secp_generator_point();
        let point = self.to_point();

        let result = generator.scalar_mul(u.num()) + point.scalar_mul(v.num());

        result.x.unwrap().num() == r.num()
    }

    pub fn to_point(&self) -> EllipticCurve {
        let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);

        EllipticCurve::new(
            self.x.clone(),
            self.y.clone(),
            FieldElement::new(Integer::ZERO, prime.clone()),
            FieldElement::new(Integer::from(7), prime)
        )
    }

    pub fn order() -> Integer {
        Integer::parse_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
             16
        )
            .unwrap()
            .complete()
    }
}

pub fn secp_generator_point() -> EllipticCurve {
    let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);

    let a = FieldElement::new(Integer::ZERO, prime.clone());
    let b = FieldElement::new(Integer::from(7), prime.clone());

    let gx = Integer::parse_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap().complete();
    let gy = Integer::parse_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap().complete();

    let x = FieldElement::new(gx, prime.clone());
    let y = FieldElement::new(gy, prime.clone());

    EllipticCurve::new(
        Some(x.clone()),
        Some(y.clone()),
        a.clone(),
        b.clone()
    )
}

pub struct Signature {
    r: Integer,
    s: Integer,
}

impl Debug for Signature {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "Signature(r={}, s={})", self.r, self.s)
    }
}

impl Signature {
    pub fn new(r: Integer, s: Integer) -> Signature {
        Signature { r, s }
    }

    /// This is the Distingished Encoding Rule for encoding Signatures
    pub fn der(&self) -> String {
        let prefix = "30";

        // calculate the length of the signature
        let signature_length = self.length();
        println!("Signature length: {}", signature_length);

        let (r_length, r) = self.der_integer_length(self.r.to_digits::<u8>(Order::MsfBe));
        let (s_length, s) = self.der_integer_length(self.s.to_digits::<u8>(Order::MsfBe));
        println!("r length: {}", r_length);
        println!("s length: {}", s_length);

        prefix.to_string()
    }

    pub fn length(&self) -> usize {
        let r = self.r.to_digits::<u8>(Order::MsfBe);
        let s = self.s.to_digits::<u8>(Order::MsfBe);

        let r_len = self.der_integer_length(r);
        let s_len = self.der_integer_length(s);

        1 + 1 + r_len.0 + s_len.0
    }

    pub fn der_integer_length(&self, mut data: Vec<u8>) -> (usize, Vec<u8>) {
        let mut length = data.len();
        if data[0] & 0x80 >= 0x80 {
            println!("Data: {:?}", data);
            data.insert(0, 00);
            length += 1;
        }

        (length, data)
    }
}

#[cfg(test)]
mod tests {
    use rug::{integer::Order, Integer};

    use super::Signature;

    #[test]
    fn test_der_encryption() {
        let signature = Signature::new(
            Integer::from_str_radix("37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6", 16).unwrap(),
            Integer::from_str_radix("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec", 16).unwrap()
        );

        let der_r = signature.der_integer_length(signature.r.to_digits::<u8>(Order::MsfBe));
        println!("DER R length: {}", der_r.0);
        println!("DER R value: {:?}", der_r.1);
    }
}