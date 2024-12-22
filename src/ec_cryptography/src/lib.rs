use std::{fmt::Debug, ops::Add};
use rug::{integer::Order, ops::{Pow, RemRounding}, Integer};

mod s256_field;
pub mod traits;

use traits::Serializer;
use finite_fields::FieldElement;

#[derive(Clone)]
pub struct EllipticCurve {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    // a and b are constants of the EC
    a: FieldElement,
    b: FieldElement,
}

impl Debug for EllipticCurve {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            "EllipticCurve({:?}, {:?})", 
            self.x.clone().unwrap_or_else(|| FieldElement::new(Integer::ZERO, Integer::from(3))), 
            self.y.clone().unwrap_or(FieldElement::new(Integer::ZERO, Integer::from(3)))
        )
    }
}

impl EllipticCurve {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> EllipticCurve {
        if x.is_none() && y.is_none() {
            return EllipticCurve { x: None, y: None, a, b };
        }

        let point = EllipticCurve { x, y, a, b };
        // Ensure that the point is on the curve
        assert!(point.is_valid(), "Point is not on the curve");
    
        point
    }

    pub fn prime(&self) -> Integer {
        self.a.order()
    }

    pub fn is_valid(&self) -> bool {
        if self.x.is_none() || self.y.is_none() {
            return true;
        }
        self.y.clone().unwrap().pow(Integer::from(2)).unwrap() == 
            (self.x.clone().unwrap().pow(Integer::from(3))).unwrap() + (self.a.clone() * self.x.clone().unwrap()) + self.b.clone()
    }

    pub fn slope(&self, other: EllipticCurve) -> Option<FieldElement> {
        // Implement the slope of the curve
        if self.x.is_none() && other.x.is_none() {
            return None;
        }

        let numerator = (other.y.unwrap().num() - self.y.clone().unwrap().num()).rem_euc(self.prime());
        let denominator = (other.x.unwrap().num() - self.x.clone().unwrap().num()).rem_euc(self.prime());

        let slope = FieldElement::new(numerator, self.prime()) / FieldElement::new(denominator, self.prime());
        Some(slope)
    }

    pub fn tangent_slope(&self) -> Option<FieldElement> {
        // Implement the slope of the tangent line
        if self.x.is_none() {
            return None;
        }

        let x_pow = self.x.clone().unwrap().num().pow(2).rem_euc(self.prime());
        let numerator: Integer = ((Integer::from(3) * x_pow).rem_euc(self.prime()) + self.a.clone().num()).rem_euc(self.prime());
        let denominator: Integer = (Integer::from(2) * self.y.clone().unwrap().num()).rem_euc(self.prime());

        let slope = FieldElement::new(numerator, self.prime()) / FieldElement::new(denominator, self.prime());
        Some(slope)
    }

    pub fn identity(&self) -> Self {
        EllipticCurve::new(
            None,
            None,
            self.a.clone(),
            self.b.clone()
        )
    }

    pub fn scalar_mul(&self, coefficient: Integer) -> EllipticCurve {
        let mut current = self.clone();
        let mut result = EllipticCurve::new(None, None, self.a.clone(), self.b.clone());
        let mut scalar = coefficient;

        while scalar > Integer::ZERO {
            if (&scalar & Integer::from(1)) == Integer::from(1) {
                result = result + current.clone();
            }

            current = current.clone() + current.clone();

            scalar >>= 1;
        }
        result
    }

    pub fn secp_point(x: Integer, y: Integer) -> EllipticCurve {
        let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        
        EllipticCurve::new(
            Some(FieldElement::new(x, prime.clone())),
            Some(FieldElement::new(y, prime.clone())),
            FieldElement::new(Integer::ZERO, prime.clone()),
            FieldElement::new(Integer::from(7), prime.clone())
        )
    }
}

pub fn reverse_bits(number: Integer) -> String {
    // convert the Integer to its binary representation
    let binaries = number.to_string_radix(2);

    // reverse the binary representation
    let reversed = binaries.chars().rev().collect();

    reversed
}

impl Eq for EllipticCurve {}

impl PartialEq for EllipticCurve {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x 
            && self.y == other.y 
            && self.a == other.a 
            && self.b == other.b
    }
}

impl Add for EllipticCurve {
    type Output = EllipticCurve;

    fn add(self, other: EllipticCurve) -> EllipticCurve {
        // Ensure that the 2 Points are on the same curve, i.e. they have the same a and b
        assert!(self.a == other.a && self.b == other.b, "Points are not on the same curve");
        
        // If other is the the Identity Point (or the Point at Infinity)
        if other.x.is_none() {
            return self;
        }

        // Similarly, if self is the Identity Point, return other
        if self.x.is_none() {
            return other;
        }

        // if self.x != other.x
        if self.x != other.x {
            // calculate the slope of the line
            // There will be a definite slope because neither of the point is the Identity Point
            let slope = self.slope(other.clone()).unwrap();

            let x1 = self.x.clone().unwrap().num();
            let x2 = other.x.unwrap().num();
            let y1 = self.y.clone().unwrap().num();

            // calculate the x-coordinate of the third point
            let x3 = (slope.pow(Integer::from(2)).unwrap().num() - x1.clone() - x2).rem_euc(self.prime());
            let y3 = ((slope.num() * (x1 - x3.clone())).rem_euc(self.prime()) - y1).rem_euc(self.prime());

            return EllipticCurve::new(
                Some(FieldElement::new(x3, self.prime())),
                Some(FieldElement::new(y3, self.prime())),
                self.a.clone(),
                self.b.clone()
            );
        }

        // if self == other
        if self == other {
            if self.y.clone().unwrap().num() == Integer::from(0) {
                return self.identity();
            }

            // If the points are the same, then we need to find the tangent slope
            let slope = self.tangent_slope().unwrap();

            let x1 = self.x.clone().unwrap().num();
            let y1 = self.y.clone().unwrap().num();
            
            let x3 = (slope.num().pow(2).rem_euc(self.prime()) - (Integer::from(2) * x1.clone()).rem_euc(self.prime())).rem_euc(self.prime());
            let y3 = ((slope.num() * (x1 - x3.clone())).rem_euc(self.prime()) - y1).rem_euc(self.prime());

            return EllipticCurve::new(
                Some(FieldElement::new(x3, self.prime())),
                Some(FieldElement::new(y3, self.prime())),
                self.a.clone(),
                self.b.clone()
            );
        }

        self.identity()
    }
}

impl traits::Serializer for EllipticCurve {
    fn sec(&self) -> Vec<u8> {
        // Uncompressed format serialization of a a pubkey
        let prefix = b"0x04";
        let serialized_x = self.x.clone().unwrap().num().to_digits::<u8>(Order::LsfLe);
        let serialized_y = self.y.clone().unwrap().num().to_digits::<u8>(Order::LsfLe);

        let mut serialized = Vec::new();
        serialized.extend_from_slice(prefix);
        serialized.extend_from_slice(&serialized_x);
        serialized.extend_from_slice(&serialized_y);

        serialized
    }
}

#[cfg(test)]
mod tests {
    use std::{panic, result};

    use finite_fields::FieldElement;
    use rug::{ops::Pow, Complete, Integer};

    use crate::{s256_field::secp_generator_point, EllipticCurve};

    #[test]
    fn test_on_curve() {
        let prime = Integer::from(223);
        let a = FieldElement::new(Integer::from(0), prime.clone());
        let b = FieldElement::new(Integer::from(7), prime.clone());

        let valid_points = [
            (Integer::from(192), Integer::from(105)), 
            (Integer::from(17), Integer::from(56)), 
            (Integer::from(1), Integer::from(193))
        ];
        let invalid_points = [
            (Integer::from(200), Integer::from(119)), 
            (Integer::from(42), Integer::from(99))
        ];

        for (x_raw, y_raw) in valid_points.iter() {
            let x = FieldElement::new(x_raw.clone(), prime.clone());
            let y = FieldElement::new(y_raw.clone(), prime.clone());
            assert_eq!(EllipticCurve::new(Some(x), Some(y), a.clone(), b.clone()).is_valid(), true);
        }

        for (x_raw, y_raw) in invalid_points.iter() {
            let x = FieldElement::new(x_raw.clone(), prime.clone());
            let y = FieldElement::new(y_raw.clone(), prime.clone());

            let result = panic::catch_unwind(|| {
                EllipticCurve::new(Some(x), Some(y), a.clone(), b.clone()).is_valid()
            });
            assert!(result.is_err(), "Point is not on the curve");
        }
    }

    #[test]
    fn test_add() {
        let prime = Integer::from(223);

        let a = FieldElement::new(Integer::from(0), prime.clone());
        let b = FieldElement::new(Integer::from(7), prime.clone());

        let points = [
            (
                FieldElement::new(Integer::from(192), prime.clone()), 
                FieldElement::new(Integer::from(105), prime.clone()),
                FieldElement::new(Integer::from(17), prime.clone()), 
                FieldElement::new(Integer::from(56), prime.clone()),
                FieldElement::new(Integer::from(170), prime.clone()), 
                FieldElement::new(Integer::from(142), prime.clone())
            ),
            (
                FieldElement::new(Integer::from(47), prime.clone()), 
                FieldElement::new(Integer::from(71), prime.clone()),
                FieldElement::new(Integer::from(117), prime.clone()), 
                FieldElement::new(Integer::from(141), prime.clone()),
                FieldElement::new(Integer::from(60), prime.clone()), 
                FieldElement::new(Integer::from(139), prime.clone())
            ),
            (
                FieldElement::new(Integer::from(143), prime.clone()), 
                FieldElement::new(Integer::from(98), prime.clone()),
                FieldElement::new(Integer::from(76), prime.clone()), 
                FieldElement::new(Integer::from(66), prime.clone()),
                FieldElement::new(Integer::from(47), prime.clone()), 
                FieldElement::new(Integer::from(71), prime)
            ),
        ];

        for (x1, y1, x2, y2, x3, y3) in points {
            let point_a = EllipticCurve::new(Some(x1), Some(y1), a.clone(), b.clone());
            let point_b = EllipticCurve::new(Some(x2), Some(y2), a.clone(), b.clone());
            let point_3 = point_a.clone() + point_b.clone();

            assert_eq!(point_3.x.unwrap().num(), x3.num());
            assert_eq!(point_3.y.unwrap().num(), y3.num());
        }
    }

    #[test]
    fn test_secp256_point() {
        let prime = Integer::from(2).pow(256) - Integer::from(2).pow(32) - Integer::from(977);
        println!("The calculated prime for secp256 is {}", prime);

        let a = FieldElement::new(Integer::ZERO, prime.clone());
        let b = FieldElement::new(Integer::from(7), prime.clone());

        let gx = Integer::parse_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap().complete();
        let gy = Integer::parse_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap().complete();

        let scalar = Integer::parse_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap().complete();

        let x = FieldElement::new(gx, prime.clone());
        let y = FieldElement::new(gy, prime.clone());

        // let point = 
        let result = panic::catch_unwind(|| {
            EllipticCurve::new(
                Some(x.clone()),
                Some(y.clone()),
                a.clone(),
                b.clone()
            ).is_valid()
        });
        assert!(result.is_ok(), "Point is not on the curve");

        // let new_point = EllipticCurve::new(
        //     Some(x),
        //     Some(y),
        //     a.clone(),
        //     b.clone()
        // ).scalar_mul(scalar.clone().to_u64().unwrap());

        // println!()
        
    }

    #[test]
    fn test_secp_signature() {
        /*
        ** Given (r, s) which are coordinates of our signature,
        ** Given z (hash of the thing being signed) and,
        ** P as the plublic key of the signer
        ** We need to calculate u and v equal to _(z/s)_ and _(r/s)_ respectively
        ** We then calculate the point uG + vP = R
        ** R.x is equal to r
         */
        let order = Integer::parse_radix(
            "fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 
            16)
        .unwrap().
        complete();     // The order of the secp256k1 curve. At multiple of this value, the curve become Infinity.
        let generator_point = secp_generator_point();

        let point_x = Integer::parse_radix("04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574", 16).unwrap().complete();
        let point_y = Integer::from_str_radix("82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4", 16).unwrap();

        let point = EllipticCurve::secp_point(
            point_x, 
            point_y
        );

        let z = FieldElement::new(
            Integer::from_str_radix("bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423", 16).unwrap(),
            order.clone()
        );
        let r = FieldElement::new(
            Integer::parse_radix(
                "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6", 
                16)
                .unwrap()
                .complete(),
            order.clone()
        );
        let s = FieldElement::new(Integer::parse_radix("8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec", 16).unwrap().complete(), order.clone());

        let u = z / s.clone();
        let v = r.clone() / s;
        
        let u_point = generator_point.scalar_mul(u.num());
        let v_point = point.scalar_mul(v.num());

        let result = u_point + v_point;

        assert_eq!(result.x.unwrap().num(), r.num(), "Points should be equal");
    }
}