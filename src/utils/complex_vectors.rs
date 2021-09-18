use std::{fmt::Display, ops::{Add, Mul, Neg}};

use crate::utils::complex_numbers::Complex;

/// Newtype pattern for complex vectors.
#[derive(Debug, PartialEq)]
pub struct ComplexVector(pub Vec<Complex>);

/// Support for adding complex vectors.
impl Add for ComplexVector {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add_vectors(self, rhs)
    }
}

/// Support for scalar product on complex vectors.
impl Mul<Complex> for ComplexVector {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        product_vector_scalar(self, rhs)
    }
}

/// Support for negating complex vectors.
impl Neg for ComplexVector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        inverse_vector(self)
    }
}

/// Support for displaying complex vectors.
impl Display for ComplexVector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_string = self.0.iter()
                                  .map(|c| c.to_string())
                                  .fold(String::new(), |acc, c| acc + &c + ", ");

        // Remove the last two chars, standing for the last separator ", "
        write!(f, "[{}]", &result_string[0..result_string.len()-2])
    }
}

/// Coordinate-wise vector addition.
fn add_vectors(ComplexVector(lhs): ComplexVector, ComplexVector(rhs): ComplexVector) -> ComplexVector {
    if lhs.len() != rhs.len() {
        panic!("Cannot add vectors of different size.");
    }

    let result_vector: Vec<Complex> = lhs.iter().zip(rhs.iter())
                                                .map(|(&x,&y)| x + y)
                                                .collect();

    ComplexVector(result_vector)
}

/// Coordinate-wise complex scalar by complex vector product.
fn product_vector_scalar(ComplexVector(vector): ComplexVector, scalar: Complex) -> ComplexVector {
    ComplexVector(vector.iter().map(|&x| x * scalar).collect())
}

/// Inverse over addition vector, by negating each coordinate.
fn inverse_vector(ComplexVector(vector): ComplexVector) -> ComplexVector {
    ComplexVector(vector.iter().map(|&x| -x).collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_vector_add_different_size() {
        let v1 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0)]);
        let v2 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1)]);
        v1 + v2;
    }

    #[test]
    fn test_vector_add() {
        let v1 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
        let v2 = ComplexVector(vec![Complex::new(16.0, 2.5), Complex::new(0.0, -7.0), Complex::new(6.0, 0.0), Complex::new(0.0, -4.0)]);
        let v3 = ComplexVector(vec![Complex::new(22.0, -1.5), Complex::new(7.0, -4.0), Complex::new(10.2, -8.1), Complex::new(0.0, -7.0)]);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_vector_product_scalar() {
        let v1 = ComplexVector(vec![Complex::new(6.0, 3.0), Complex::new(0.0, 0.0), Complex::new(5.0, 1.0), Complex::new(4.0, 0.0)]);
        let v2 = ComplexVector(vec![Complex::new(12.0, 21.0), Complex::new(0.0, 0.0), Complex::new(13.0, 13.0), Complex::new(12.0, 8.0)]);

        assert_eq!(v1 * Complex::new(3.0, 2.0), v2);
    }

    #[test]
    fn test_vector_inverse() {
        let v1 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
        let v2 = ComplexVector(vec![Complex::new(-6.0, 4.0), Complex::new(-7.0, -3.0), Complex::new(-4.2, 8.1), Complex::new(0.0, 3.0)]);

        assert_eq!(-v1, v2);
    }
}
