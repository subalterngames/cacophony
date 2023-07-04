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

impl Div<u64> for Fraction {
    type Output = u64;

    fn div(self, rhs: u64) -> Self::Output {
        (rhs * self.denominator) / self.numerator
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
