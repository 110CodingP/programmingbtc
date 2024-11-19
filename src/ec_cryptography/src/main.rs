use ec_cryptography::EllipticCurve;
use finite_fields::FieldElement;

fn main() {
    let prime = 223;
    let x = Some(FieldElement::new(1, prime));
    let y = Some(FieldElement::new(193, prime));
    let a = FieldElement::new(0, prime);
    let b = FieldElement::new(7, prime);

    let point_a: EllipticCurve = EllipticCurve::new(x, y, a, b);

    println!("Point A: {:?} is a valid Point, {}", point_a, point_a.is_valid());
}
