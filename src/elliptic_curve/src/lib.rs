use std::{fmt::Debug, ops::Add};

/// While coding an elliptic curve, we are mostly interested in the Point on the curve.
/// The points suffice because they will form a finite field which is useful in ECC operations
#[derive(Clone)]
pub struct EllipticPoint {
    x: Option<i64>,
    y: Option<i64>,
    // a and b are constants of the EC
    a: i64,
    b: i64,
}

impl Debug for EllipticPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "EllipticPoint({}, {})", self.x.unwrap_or(0), self.y.unwrap_or(0))
    }
}

impl EllipticPoint {
    pub fn new(
        x: Option<i64>,
        y: Option<i64>,
        a: i64,
        b: i64,
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
        self.y.unwrap().pow(2) == (self.x.unwrap().pow(3)) + (self.a * self.x.unwrap()) + self.b
    }

    pub fn slope(&self, other: EllipticPoint) -> Option<i64> {
        // Implement the slope of the curve
        if self.x.is_none() && other.x.is_none() {
            return None;
        }

        let slope = (other.y.unwrap() - self.y.unwrap()) / (other.x.unwrap() - self.x.unwrap());
        Some(slope)
    }

    pub fn tangent_slope(&self) -> Option<i64> {
        // Implement the slope of the tangent line
        if self.x.is_none() {
            return None;
        }

        let slope = (3 * self.x.unwrap().pow(2) + self.a) / (2 * self.y.unwrap());
        Some(slope)
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
            // If the points are the same, then we need to find the tangent slope
            let slope = self.tangent_slope().unwrap();
            let x_3 = slope.pow(2) - (2 * self.x.unwrap());
            let y_3 = (slope * (self.x.unwrap() - x_3)) - self.y.unwrap();

            let point_3 = EllipticPoint::new(Some(x_3), Some(y_3), self.a, self.b);
            return point_3;
        }

        // If x1 != x2
        // find x3 = s(exp)2 - x1 - x2
        let x_3 = self.clone().slope(other.clone()).unwrap().pow(2) - self.x.unwrap() - other.x.unwrap();

        let y_3 = self.clone().slope(other.clone()).unwrap() * (self.x.unwrap() - x_3) - self.y.unwrap();

        let point_3 = EllipticPoint::new(Some(x_3), Some(y_3), self.a, self.b);
        point_3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inequality() {
        let a = EllipticPoint::new(Some(3), Some(7), 5, 7);
        let b = EllipticPoint::new(Some(18), Some(77), 5, 7);

        assert_eq!(a, a, "Points are not the same");
        assert_ne!(a, b, "Points are the same");
    }

    #[test]
    fn test_addition_vertical() {
        let a = EllipticPoint::new(None, None, 5, 7);
        let b = EllipticPoint::new(Some(2), Some(5), 5, 7);
        let c = EllipticPoint::new(Some(2), Some(-5), 5, 7);

        assert_eq!(a.clone() + b.clone(), b, "Vertical addition failed");
        assert_eq!(b.clone() + a.clone(), b, "Vertical addition failed");
        assert_eq!(b + c, a, "Vertical addition failed");
    }

    #[test]
    fn test_addition_1() {
        let a = EllipticPoint::new(Some(3), Some(7), 5, 7);
        let b = EllipticPoint::new(Some(-1), Some(-1), 5, 7);

        assert_eq!(a + b, EllipticPoint::new(Some(2), Some(-5), 5, 7), "Addition failed");
    }

    #[test]
    fn test_addition_2() {
        let a = EllipticPoint::new(Some(-1), Some(-1), 5, 7);

        assert_eq!(a.clone() + a.clone(), EllipticPoint::new(Some(18), Some(77), 5, 7), "Addition failed");
    }
}