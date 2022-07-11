use std::ops::{Add, Mul, Neg, Sub, Div, AddAssign};
use std::iter::Sum;
use std::fmt::{Formatter, Result, Display};
use std::convert::From;

/// Polar coordinates representation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Polar(pub f64, pub f64);

impl From<Complex> for Polar {
    fn from(Complex { real: r, imaginary: i }: Complex) -> Self {
        Polar(f64::sqrt(f64::powi(r, 2) + f64::powi(i, 2)), f64::atan(i / r))
    }
}

impl From<Cartesian> for Polar {
    fn from(Cartesian(x, y): Cartesian) -> Self {
        Polar(f64::sqrt(f64::powi(x, 2) + f64::powi(y, 2)), f64::atan(y / x))
    }
}

impl Display for Polar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// Cartesian coordinates representation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Cartesian(pub f64, pub f64);

impl From<Complex> for Cartesian {
    fn from(Complex { real: r, imaginary: i }: Complex) -> Self {
        Cartesian(r, i)
    }
}

impl From<Polar> for Cartesian {
    fn from(Polar(magnitude, phase): Polar) -> Self {
        Cartesian(magnitude * f64::cos(phase), magnitude * f64::sin(phase))
    }
}

impl Display for Cartesian {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

/// Representation of a Complex number.
/// A more robust implementation would probably use a generic
/// numeric type for the fields, but always using f64 will
/// always do for my purposes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Complex {
    /// The real part of the complex number.
    pub real: f64,
    /// The imaginary part of the complex number.
    pub imaginary: f64,
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

impl From<Polar> for Complex {
    fn from(polar: Polar) -> Self {
        Complex::from(Cartesian::from(polar))
    }
}

impl From<Cartesian> for Complex {
    fn from(Cartesian(x, y): Cartesian) -> Self {
        Self::new(x, y)
    }
}

// Support for adding complex numbers.
impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.real + other.real, self.imaginary + other.imaginary)
    }
}

// Support for assigning addition result of complex numbers.
impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        };
    }
}

// Support for getting the sum of an iterator of complex numbers.
impl Sum for Complex {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item=Self> {
        iter.fold(Complex::new(0.0, 0.0), |acc, x| acc + x)
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

    #[test]
    fn test_cartesian_to_polar() {
        assert_eq!(Polar::from(Cartesian(1.0, 1.0)), Polar(f64::sqrt(2.0), f64::atan(1.0)));
    }

    #[test]
    fn test_polar_to_cartesian() {
        let Cartesian(x, y) = Cartesian::from(Polar(f64::sqrt(2.0), f64::atan(1.0)));

        // Allow for some rounding errors.
        assert!(f64::abs(x - 1.0) < 0.01 && f64::abs(y - 1.0) < 0.01);
    }
}
