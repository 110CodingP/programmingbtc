use ec_cryptography::{EllipticCurve, reverse_bits, traits::Serializer};
use finite_fields::FieldElement;
use rug::Integer;
use rug::{ops::Pow, Complete};

fn main() {
    // let prime = Integer::from(223);

    // let a = FieldElement::new(Integer::from(0), prime.clone());
    // let b = FieldElement::new(Integer::from(7), prime.clone());

    // let point = EllipticCurve::new(
    //     Some(FieldElement::new(Integer::from(47), prime.clone())),
    //     Some(FieldElement::new(Integer::from(71), prime.clone())),
    //     a.clone(), b.clone()
    // );
    // let pointb = EllipticCurve::new(
    //     Some(FieldElement::new(Integer::from(36), prime.clone())),
    //     Some(FieldElement::new(Integer::from(111), prime.clone())),
    //     a, b
    // );

    // println!("Point is valid, {}", point.is_valid());

    // // let mut result = point.identity();
    // let mut current = point.clone();

    // // current = current.clone() + current.clone();
    // // result = current.clone() + result.clone();

    // let point_x4 = point.scalar_mul(21);


    // The uncompressed SEC format for the public key where private key is 5000

    // compose the Generator point for Bitcoin
    let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);

    let a = FieldElement::new(Integer::ZERO, prime.clone());
    let b = FieldElement::new(Integer::from(7), prime.clone());

    let gx = Integer::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap();
    let gy = Integer::parse_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap().complete();

    let x = FieldElement::new(gx, prime.clone());
    let y = FieldElement::new(gy, prime.clone());

    // let point = 
    let generator = EllipticCurve::new(
        Some(x.clone()),
        Some(y.clone()),
        a.clone(),
        b.clone()
    );
    // let scalar = Integer::parse_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap().complete();

    let public_key_5000 = generator.scalar_mul(Integer::from(5000));
    let serialized_key = public_key_5000.sec();
    let serialized_key_string: String = serialized_key.iter().map(|byte| format!("{:02x}", byte)).collect();

    println!("Serialized Public key for 5000 is: {:?}", serialized_key_string);
}
