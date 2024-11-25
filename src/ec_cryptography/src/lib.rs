use std::{fmt::Debug, ops::Add};
use rug::{ops::Pow, Integer};

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
            self.x.clone().unwrap(), 
            self.y.clone().unwrap()
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

    pub fn is_valid(&self) -> bool {
        if self.x.is_none() || self.y.is_none() {
            return true;
        }
        self.y.clone().unwrap().pow(2) == (self.x.clone().unwrap().pow(3)) + (self.a.clone() * self.x.clone().unwrap()) + self.b.clone()
    }

    pub fn slope(&self, other: EllipticCurve) -> Option<FieldElement> {
        // Implement the slope of the curve
        if self.x.is_none() && other.x.is_none() {
            return None;
        }

        let slope = (other.y.unwrap() - self.y.clone().unwrap()) / (other.x.unwrap() - self.x.clone().unwrap());
        Some(slope)
    }

    pub fn tangent_slope(&self) -> Option<i32> {
        // Implement the slope of the tangent line
        if self.x.is_none() {
            return None;
        }

        let slope: Integer = (3 * self.x.clone().unwrap().num().pow(2) + self.a.num()) / (2 * self.y.clone().unwrap().num());
        Some(slope.to_i32_wrapping())
    }
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

        if self.x == other.x && self.y != other.y {
            // If the x-coordinates are the same but the y-coordinates are different, then the result is the Identity Point
            return EllipticCurve::new(None, None, self.a, self.b);
        }

        if self == other {
            if self.y.clone().unwrap().num() == 0 {
                return EllipticCurve::new(None, None, self.a, self.b);
            }
            // If the points are the same, then we need to find the tangent slope
            let slope = self.tangent_slope().unwrap();
            let x_3: Integer = slope.pow(2) - (2 * self.x.clone().unwrap().num());
            let y_3 = (slope * (self.x.clone().unwrap().num() - x_3.clone())) - self.y.clone().unwrap().num();

            let point_3 = EllipticCurve::new(
                Some(FieldElement::new(x_3, self.x.unwrap().order())), 
                Some(FieldElement::new(y_3, self.y.unwrap().order())), 
                self.a, 
                self.b
            );
            return point_3;
        }

        // If x1 != x2
        // find x3 = s(exp)2 - x1 - x2
        let x_3 = self.clone().slope(other.clone()).unwrap().pow(2) - self.x.clone().unwrap() - other.clone().x.unwrap();

        let y_3 = self.clone().slope(other.clone()).unwrap() * (self.x.clone().unwrap() - x_3.clone()) - self.y.unwrap();

        let point_3 = EllipticCurve::new(
            Some(x_3), 
            Some(y_3), 
            self.a, 
            self.b
        );
        point_3
    }
}

#[cfg(test)]
mod tests {
    use std::panic;

    use finite_fields::FieldElement;
    use rug::Integer;

    use crate::EllipticCurve;

    #[test]
    fn test_on_curve() {
        let prime = Integer::from(223_i16);
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
}