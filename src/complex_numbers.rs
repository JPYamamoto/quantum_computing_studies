use std::ops::{Add, Mul, Neg, Sub, Div};
use std::fmt::{Formatter, Result, Display};

pub fn programming_drill_1_1_1() {
    println!("Solution to the programming drill 1.1.1.");

    let c1 = Complex::new(-1.0, 3.0);
    let c2 = Complex::new(9.0, -5.0);

    println!("({}) + ({}) = {}", c1, c2, c1 + c2);
    println!("({}) * ({}) = {}", c1, c2, c1 * c2);
}

pub fn programming_drill_1_2_1() {
    println!("Solution to the programming drill 1.2.1.");

    let c1 = Complex::new(9.0, 3.0);
    let c2 = Complex::new(10.0, -5.0);

    println!("({}) - ({}) = {}", c1, c2, c1 - c2);
    println!("({}) / ({}) = {}", c1, c2, c1 / c2);
    println!("|{}| = {}", c1, Complex::abs(c1));
    println!("Conj[{}] = {}", c1, Complex::conjugate(c1));
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
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self {real, imaginary}
    }

    pub fn abs(self) -> f64 {
        let Complex { real: r, imaginary: i } = self;
        f64::sqrt(f64::powi(r, 2) + f64::powi(i, 2))
    }

    pub fn conjugate(self) -> Self {
        let Complex { real: r, imaginary: i } = self;
        Self::new(r, -i)
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

// Support for negating complex numbers.
impl Neg for Complex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.real, -self.imaginary)
    }
}

// Support for subtracting complex numbers.
impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

// Support for dividing complex numbers.
impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        if other.real == 0.0 && other.imaginary == 0.0 {
            panic!("Cannot divide by zero!");
        }

        let Complex { real: r1, imaginary: i1 } = self;
        let Complex { real: r2, imaginary: i2 } = other;

        let real_part = ((r1 * r2) + (i1 * i2)) / (f64::powi(r2, 2) + f64::powi(i2, 2));
        let imaginary_part = ((r2 * i1) - (r1 * i2)) / (f64::powi(r2, 2) + f64::powi(i2, 2));

        Self::new(real_part, imaginary_part)
    }
}

// Support for displaying complex numbers.
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
    fn test_add_neutral() {
        let number1 = Complex::new(9.6, -3.4);
        let number2 = Complex::new(2.0, 8.0);
        assert_eq!(number1 + Complex::new(0.0, 0.0), number1);
        assert_eq!(number2 + Complex::new(0.0, 0.0), number2);
    }

    #[test]
    fn test_mul() {
        assert_eq!(Complex::new(-3.0, 1.0) * Complex::new(2.0, -4.0), Complex::new(-2.0, 14.0));
    }

    #[test]
    fn test_mul_identity() {
        let number1 = Complex::new(9.6, -3.4);
        let number2 = Complex::new(2.0, 8.0);
        assert_eq!(number1 * Complex::new(1.0, 0.0), number1);
        assert_eq!(number2 * Complex::new(1.0, 0.0), number2);
    }

    #[test]
    fn test_div() {
        assert_eq!(Complex::new(-2.0, 1.0) / Complex::new(1.0, 2.0), Complex::new(0.0, 1.0));
        assert_eq!(Complex::new(0.0, 3.0) / Complex::new(-1.0, -1.0), Complex::new(-1.5, -1.5));
    }

    #[test]
    fn test_neg() {
        assert_eq!(-Complex::new(-3.0, 1.0), Complex::new(3.0, -1.0));
        assert_eq!(-Complex::new(2.5, 4.8), Complex::new(2.5, 4.8) * Complex::new(-1.0, 0.0));
    }

    #[test]
    fn test_abs() {
        assert_eq!(Complex::abs(Complex::new(4.0, -3.0)), 5.0);
    }

    #[test]
    fn test_conjugate() {
        assert_eq!(Complex::conjugate(Complex::new(4.0, -3.0)), Complex::new(4.0, 3.0));
        assert_eq!(Complex::conjugate(Complex::new(0.0, 5.0)), Complex::new(0.0, -5.0));
        assert_eq!(Complex::conjugate(Complex::new(1.0, 0.0)), Complex::new(1.0, 0.0));
    }
}
