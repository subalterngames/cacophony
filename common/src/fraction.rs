use crate::U64orF32;
use std::fmt::Display;
use std::ops::{Div, Mul};

/// A fraction has a numerator and denominator and can be multiplied or divided.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}

impl Fraction {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    /// Flip the numerator and denominator.
    pub fn invert(&mut self) {
        std::mem::swap(&mut self.numerator, &mut self.denominator);
    }
}

impl From<u64> for Fraction {
    fn from(value: u64) -> Self {
        Self::new(value, 1)
    }
}

impl From<U64orF32> for Fraction {
    fn from(value: U64orF32) -> Self {
        Self::new(value.get_u(), 1)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl Mul<u64> for Fraction {
    type Output = u64;

    fn mul(self, rhs: u64) -> Self::Output {
        (rhs * self.numerator) / self.denominator
    }
}

impl Mul<Fraction> for u64 {
    type Output = u64;

    fn mul(self, rhs: Fraction) -> Self::Output {
        (self * rhs.numerator) / rhs.denominator
    }
}

impl Mul<Fraction> for Fraction {
    type Output = Fraction;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul<Fraction> for U64orF32 {
    type Output = U64orF32;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Self::from(self.get_u() * rhs)
    }
}

impl Mul<U64orF32> for Fraction {
    type Output = U64orF32;

    fn mul(self, rhs: U64orF32) -> Self::Output {
        Self::Output::from(self * rhs.get_u())
    }
}

impl Div<u64> for Fraction {
    type Output = u64;

    fn div(self, rhs: u64) -> Self::Output {
        (rhs * self.numerator) / self.denominator
    }
}

impl Div<Fraction> for u64 {
    type Output = u64;

    fn div(self, rhs: Fraction) -> Self::Output {
        (self * rhs.denominator) / rhs.numerator
    }
}

impl Div<Fraction> for Fraction {
    type Output = Fraction;

    fn div(self, rhs: Fraction) -> Self::Output {
        Self::new(
            self.numerator * rhs.denominator,
            self.denominator * rhs.numerator,
        )
    }
}

impl Div<Fraction> for U64orF32 {
    type Output = U64orF32;

    fn div(self, rhs: Fraction) -> Self::Output {
        Self::from(self.get_u() / rhs)
    }
}

impl Div<U64orF32> for Fraction {
    type Output = U64orF32;

    fn div(self, rhs: U64orF32) -> Self::Output {
        Self::Output::from(self / rhs.get_u())
    }
}

#[cfg(test)]
mod tests {
    use crate::fraction::Fraction;
    use crate::U64orF32;

    #[test]
    fn fraction() {
        // Instantiation.
        let mut fr = Fraction::from(3);
        assert_eq!(fr.numerator, 3);
        assert_eq!(fr.denominator, 1);
        fr = Fraction::new(1, 2);
        assert_eq!(fr.numerator, 1);
        assert_eq!(fr.denominator, 2);
        // Inversion.
        fr.invert();
        assert_eq!(fr.numerator, 2);
        assert_eq!(fr.denominator, 1);
        // Equality.
        fr.numerator = 3;
        fr.denominator = 5;
        // u64.
        let u = 7;
        assert_eq!(u * fr, 4);
        assert_eq!(fr * u, 4);
        assert_eq!(u / fr, 11);
        assert_eq!(fr / u, 4);
        // Fraction.
        let fr1 = Fraction::new(1, 2);
        let mut fr2 = fr * fr1;
        assert_eq!(fr2.numerator, 3);
        assert_eq!(fr2.denominator, 10);
        fr2 = fr1 * fr;
        assert_eq!(fr2.numerator, 3);
        assert_eq!(fr2.denominator, 10);
        fr2 = fr / fr1;
        assert_eq!(fr2.numerator, 6);
        assert_eq!(fr2.denominator, 5);
        fr2 = fr1 / fr;
        assert_eq!(fr2.numerator, 5);
        assert_eq!(fr2.denominator, 6);
        // U64orF32.
        let uf = U64orF32::from(7);
        assert_eq!((uf * fr).get_u(), 4);
        assert_eq!((fr * uf).get_u(), 4);
        assert_eq!((uf / fr).get_u(), 11);
        assert_eq!((fr / uf).get_u(), 4);
    }
}
