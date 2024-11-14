use elliptic_curve::EllipticPoint;

fn main() {
    // Invalid
    // let point_1 = EllipticPoint::new(Some(2), Some(4), 5, 7);
    // println!("Point 1 is valid: {}", point_1.is_valid());


    let point_2 = EllipticPoint::new(Some(-1), Some(-1), 5, 7);
    println!("Point 1 is valid: {}", point_2.is_valid());


    let point_3 = EllipticPoint::new(Some(18), Some(77), 5, 7);
    println!("Point 1 is valid: {}", point_3.is_valid());

    // Invalid
    // let point_4 = EllipticPoint::new(Some(5), Some(7), 5, 7);
    // println!("Point 1 is valid: {}", point_4.is_valid());

    let point_a = EllipticPoint::new(Some(-1), Some(-1), 5, 7);
    println!("Poin A + Point A: {:?}", point_a.clone() + point_a);
}
