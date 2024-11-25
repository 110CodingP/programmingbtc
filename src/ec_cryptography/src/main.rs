use ec_cryptography::EllipticCurve;
use finite_fields::FieldElement;
use rug::Integer;

fn main() {
    // let prime = 223;
    // let x = Some(FieldElement::new(1, prime));
    // let y = Some(FieldElement::new(193, prime));
    // let a = FieldElement::new(0, prime);
    // let b = FieldElement::new(7, prime);

    // let point_a: EllipticCurve = EllipticCurve::new(x, y, a, b);

    // println!("Point A: {:?} is a valid Point, {}", point_a, point_a.is_valid());

    // Addition of finite fields
    let prime = Integer::from(7);
    let x_1 = Some(FieldElement::new(Integer::from(170), prime.clone()));
    let x_2 = Some(FieldElement::new(Integer::from(60), prime.clone()));

    let y_1 = Some(FieldElement::new(Integer::from(142), prime.clone()));
    let y_2 = Some(FieldElement::new(Integer::from(139), prime.clone()));
    let a = FieldElement::new(Integer::ZERO, prime.clone());
    let b = FieldElement::new(Integer::from(7), prime.clone());

    let point_a = EllipticCurve::new(
        x_1,
        y_1,
        a.clone(),
        b.clone()
    );
    let point_b = EllipticCurve::new(
        x_2,
        y_2,
        a,
        b
    );

    let point_3 = point_a + point_b;
    println!("Point 3: {:?}", point_3);
}
