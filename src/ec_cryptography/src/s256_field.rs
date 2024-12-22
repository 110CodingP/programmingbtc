use finite_fields::FieldElement;
use rug::{Integer, Complete};
use rug::ops::Pow;

use crate::EllipticCurve;

pub struct S256Field {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
}

impl S256Field {
    pub fn new() -> S256Field {
        S256Field { x: None, y: None }
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