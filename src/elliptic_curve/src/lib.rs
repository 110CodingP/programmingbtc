use std::{fmt::Debug, ops::Add};

use rug::integer::Order;
use rug::Integer;
use rug::ops::Pow;

use traits::Serializer;

pub mod traits;

/// While coding an elliptic curve, we are mostly interested in the Point on the curve.
/// The points suffice because they will form a finite field which is useful in ECC operations
#[derive(Clone)]
pub struct EllipticPoint {
    x: Option<Integer>,
    y: Option<Integer>,
    // a and b are constants of the EC
    a: Integer,
    b: Integer,
}

impl Debug for EllipticPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EllipticPoint({}, {})", 
            self.x.as_ref().unwrap_or(&Integer::ZERO), 
            self.y.as_ref().unwrap_or(&Integer::ZERO)
        )
    }
}

impl EllipticPoint {
    pub fn new(
        x: Option<Integer>,
        y: Option<Integer>,
        a: Integer,
        b: Integer,
    ) -> EllipticPoint {
        // If x and y are both None, then we are defining the Identity Point
        if x.is_none() && y.is_none() {
            return EllipticPoint { x: None, y: None, a, b };
        }

        let point = EllipticPoint { x, y, a, b };
        // Ensure that the point is on the curve
        assert!(point.is_valid(), "Point is not on the curve");
    
        point
    }

    pub fn is_valid(&self) -> bool {
        if self.x.is_none() || self.y.is_none() {
            return true;
        }
        self.y.clone().unwrap().pow(2) == 
            (self.x.clone().unwrap().pow(3)) + 
            (self.a.clone() * self.x.clone().unwrap()) + 
            self.b.clone()
    }

    pub fn slope(&self, other: EllipticPoint) -> Option<i64> {
        // Implement the slope of the curve
        if self.x.is_none() && other.x.is_none() {
            return None;
        }

        let slope = (other.y.unwrap() - self.y.clone().unwrap()) / (other.x.unwrap() - self.x.clone().unwrap());
        Some(slope.to_i64_wrapping())
    }

    pub fn tangent_slope(&self) -> Option<i64> {
        // Implement the slope of the tangent line
        if self.x.is_none() {
            return None;
        }

        let slope: Integer = (3 * self.x.clone().unwrap().pow(2) + self.a.clone()) / (2 * self.y.clone().unwrap());
        Some(slope.to_i64_wrapping())
    }
}

impl Eq for EllipticPoint {}

impl PartialEq for EllipticPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x 
            && self.y == other.y 
            && self.a == other.a 
            && self.b == other.b
    }
}

impl Add for EllipticPoint {
    type Output = EllipticPoint;

    fn add(self, other: EllipticPoint) -> EllipticPoint {
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

        if self.x == other.x && self.y != other.y {
            // If the x-coordinates are the same but the y-coordinates are different, then the result is the Identity Point
            return EllipticPoint::new(None, None, self.a, self.b);
        }

        if self == other {
            if self.y.clone().unwrap() == 0 {
                return EllipticPoint::new(None, None, self.a, self.b);
            }
            // If the points are the same, then we need to find the tangent slope
            let slope = self.tangent_slope().unwrap();
            let x_3: Integer = slope.pow(2) - (2 * self.x.clone().unwrap());
            let y_3 = (slope * (self.x.clone().unwrap() - x_3.clone())) - self.y.clone().unwrap();

            let point_3 = EllipticPoint::new(Some(x_3), Some(y_3), self.a, self.b);
            return point_3;
        }

        // If x1 != x2
        // find x3 = s(exp)2 - x1 - x2
        let x_3 = self.clone().slope(other.clone()).unwrap().pow(2) - self.x.clone().unwrap() - other.x.clone().unwrap();

        let y_3 = self.clone().slope(other.clone()).unwrap() * (self.x.unwrap() - x_3.clone()) - self.y.unwrap();

        let point_3 = EllipticPoint::new(Some(x_3), Some(y_3), self.a, self.b);
        point_3
    }
}

impl traits::Serializer for EllipticPoint {
    fn sec(&self) -> Vec<u8> {
        // Uncompressed format serialization of a a pubkey
        let prefix = b"0x04";
        let serialized_x = self.x.clone().unwrap().to_digits::<u8>(Order::LsfLe);
        let serialized_y = self.y.clone().unwrap().to_digits::<u8>(Order::LsfLe);

        let mut serialized = Vec::new();
        serialized.extend_from_slice(prefix);
        serialized.extend_from_slice(&serialized_x);
        serialized.extend_from_slice(&serialized_y);

        serialized
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inequality() {
        let a = EllipticPoint::new(
            Some(Integer::from(3)), 
            Some(Integer::from(7)), 
            Integer::from(5), 
            Integer::from(7)
        );
        let b = EllipticPoint::new(
            Some(Integer::from(18)), 
            Some(Integer::from(77)), 
            Integer::from(5), 
            Integer::from(7)
        );

        assert_eq!(a, a, "Points are not the same");
        assert_ne!(a, b, "Points are the same");
    }

    #[test]
    fn test_addition_vertical() {
        let a = EllipticPoint::new(
            None, 
            None, 
            Integer::from(5), 
            Integer::from(7)
        );
        let b = EllipticPoint::new(
            Some(Integer::from(2)), 
            Some(Integer::from(5)), 
            Integer::from(5), 
            Integer::from(7)
        );
        let c = EllipticPoint::new(
            Some(Integer::from(2)), 
            Some(Integer::from(-5)), 
            Integer::from(5), 
            Integer::from(7)
        );

        assert_eq!(a.clone() + b.clone(), b, "Vertical addition failed");
        assert_eq!(b.clone() + a.clone(), b, "Vertical addition failed");
        assert_eq!(b + c, a, "Vertical addition failed");
    }

    #[test]
    fn test_addition_1() {
        let a = EllipticPoint::new(
            Some(Integer::from(3)), 
            Some(Integer::from(7)), 
            Integer::from(5), 
            Integer::from(7));
        let b = EllipticPoint::new(
            Some(Integer::from(-1)), 
            Some(Integer::from(-1)), 
            Integer::from(5), 
            Integer::from(7)
        );

        let result = EllipticPoint::new(
            Some(Integer::from(2)), 
            Some(Integer::from(-5)), 
            Integer::from(5), 
            Integer::from(7)
        );

        assert_eq!(
            a + b, 
            result, 
            "Addition failed"
        );
    }

    #[test]
    fn test_addition_2() {
        let a = EllipticPoint::new(
            Some(Integer::from(-1)), 
            Some(Integer::from(-1)), 
            Integer::from(5), 
            Integer::from(7)
        );
        let b = EllipticPoint::new(
            Some(Integer::from(18)), 
            Some(Integer::from(77)), 
            Integer::from(5), 
            Integer::from(7)
        );

        assert_eq!(
            a.clone() + a.clone(), 
            b,
            "Addition failed"
        );
    }
}