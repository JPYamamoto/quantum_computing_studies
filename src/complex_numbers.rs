use std::ops::{Add, Mul};
use std::fmt::{Formatter, Result, Display};

pub fn programming_drill_1_1_1() {
    println!("Solution to the programming drill 1.1.1.");

    let c1 = Complex::new(-1.0, 3.0);
    let c2 = Complex::new(9.0, -5.0);

    println!("({}) + ({}) = {}", c1, c2, c1 + c2);
    println!("({}) * ({}) = {}", c1, c2, c1 * c2);
}

/// Representation of a Complex number.
/// A more robust implementation would probably use a generic
/// numeric type for the fields, but always using f64 will
/// always do for my purposes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    /// The real part of the complex number.
    real: f64,
    /// The imaginary part of the complex number.
    imaginary: f64,
}

impl Complex {
    /// Returns a complex number with the given real and
    /// imaginary parts.
    fn new(real: f64, imaginary: f64) -> Self {
        Self {real, imaginary}
    }
}

// Support for adding complex numbers.
impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

// Support for multiplying complex numbers.
impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let real_part = (self.real * other.real) - (self.imaginary * other.imaginary);
        let imaginary_part = (self.real * other.imaginary) + (self.imaginary * other.real);
        Self::new(real_part, imaginary_part)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let operator = if self.imaginary >= 0.0 { "+" } else { "" };
        write!(f, "{}{}{}i", self.real, operator, self.imaginary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Complex::new(-3.0, 1.0) + Complex::new(2.0, -4.0), Complex::new(-1.0, -3.0));
    }

    #[test]
    fn test_mul() {
        assert_eq!(Complex::new(-3.0, 1.0) * Complex::new(2.0, -4.0), Complex::new(-2.0, 14.0));
    }
}
