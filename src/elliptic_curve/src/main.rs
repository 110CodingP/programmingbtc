use elliptic_curve::EllipticPoint;
use rug::Integer;

fn main() {
    // Invalid
    // let point_1 = EllipticPoint::new(Some(2), Some(4), 5, 7);
    // println!("Point 1 is valid: {}", point_1.is_valid());


    let point_2 = EllipticPoint::new(
        Some(Integer::from(-1)),
         Some(Integer::from(-1)), 
         Integer::from(5), 
        Integer::from(7)
    );
    println!("Point 1 is valid: {}", point_2.is_valid());


    let point_3 = EllipticPoint::new(
        Some(Integer::from(18)), 
        Some(Integer::from(77)), 
        Integer::from(5), 
       Integer::from(7)
    );
    println!("Point 1 is valid: {}", point_3.is_valid());

    // Invalid
    // let point_4 = EllipticPoint::new(Some(5), Some(7), 5, 7);
    // println!("Point 1 is valid: {}", point_4.is_valid());

    let point_a = EllipticPoint::new(
        Some(Integer::from(-1)),
         Some(Integer::from(-1)), 
         Integer::from(5), 
        Integer::from(7)
    );
    println!("Poin A + Point A: {:?}", point_a.clone() + point_a);
}
