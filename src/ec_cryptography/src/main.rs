use ec_cryptography::{EllipticCurve, reverse_bits};
use finite_fields::FieldElement;
use rug::Integer;

fn main() {
    let prime = Integer::from(223);

    let a = FieldElement::new(Integer::from(0), prime.clone());
    let b = FieldElement::new(Integer::from(7), prime.clone());

    let point = EllipticCurve::new(
        Some(FieldElement::new(Integer::from(47), prime.clone())),
        Some(FieldElement::new(Integer::from(71), prime.clone())),
        a, b
    );

    println!("Point is valid, {}", point.is_valid());

    // let mut result = point.identity();
    // let mut current = point.clone();

    // current = current.clone() + current.clone();
    // result = current.clone() + result.clone();

    // println!("Result of scalar_mu(2), {:?}", result);
    // let point_x4 = point.scalar_mul(2);

    println!("2 times point, {:?}", point.scalar_mul(2));
}
