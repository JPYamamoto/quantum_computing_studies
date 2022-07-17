use std::{fmt::Display, ops::{Add, Sub, Mul, Neg}};

use crate::utils::complex_number::Complex;

/// Newtype pattern for complex vectors.
/// I should have probably gone with generics, but I think complex will do just
/// fine for the purposes of the book. Maybe I'll change this later if the need
/// comes up.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ComplexVector<const N: usize>(pub [Complex; N]);

impl<const N: usize> ComplexVector<N> {
    pub fn distance_to(self, rhs: ComplexVector<N>) -> f64 {
        let diff = self - rhs;
        diff.norm()
    }

    pub fn norm(self) -> f64 {
        (self * self).real.sqrt()
    }
}

/// Support for adding complex vectors.
impl<const N: usize> Add for ComplexVector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add_vectors(self, rhs)
    }
}

// Support for subtracting complex vectors.
impl<const N: usize> Sub for ComplexVector<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self + -other
    }
}

/// Support for scalar product on complex vectors.
impl<const N: usize> Mul<Complex> for ComplexVector<N> {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        product_vector_scalar(self, rhs)
    }
}

impl<const N: usize> Mul<ComplexVector<N>> for ComplexVector<N> {
    type Output = Complex;

    fn mul(self, rhs: ComplexVector<N>) -> Self::Output {
        inner_product_vector(self, rhs)
    }
}

/// Support for negating complex vectors.
impl<const N: usize> Neg for ComplexVector<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        inverse_vector(self)
    }
}

/// Support for displaying complex vectors.
impl<const N: usize> Display for ComplexVector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_string = self.0.iter()
                                  .map(|c| c.to_string())
                                  .fold(String::new(), |acc, c| acc + &c + ", ");

        // Remove the last two chars, standing for the last separator ", "
        write!(f, "[{}]", &result_string[0..result_string.len()-2])
    }
}

/// Coordinate-wise vector addition.
fn add_vectors<const N: usize>(ComplexVector(lhs): ComplexVector<N>, ComplexVector(rhs): ComplexVector<N>) -> ComplexVector<N> {
    let mut result_vector = [Complex::new(0.0, 0.0); N];

    for i in 0..N {
        result_vector[i] = lhs[i] + rhs[i];
    };

    ComplexVector(result_vector)
}

/// Coordinate-wise complex scalar by complex vector product.
fn product_vector_scalar<const N: usize>(ComplexVector(vector): ComplexVector<N>, scalar: Complex) -> ComplexVector<N> {
    ComplexVector(vector.map(|x| x * scalar))
}

/// Inner product of two complex vectors, defined as the sum of the product entry by entry
/// of the conjugate vector by another vector.
fn inner_product_vector<const N: usize>(ComplexVector(v1): ComplexVector<N>, ComplexVector(v2): ComplexVector<N>) -> Complex {
    v1.iter()
      .zip(v2.iter())
      .map(|(&x1, &x2)| x1.conjugate() * x2)
      .sum()
}

/// Inverse over addition vector, by negating each coordinate.
fn inverse_vector<const N: usize>(ComplexVector(vector): ComplexVector<N>) -> ComplexVector<N> {
    ComplexVector(vector.map(|x| -x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_add() {
        let v1 = ComplexVector([Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
        let v2 = ComplexVector([Complex::new(16.0, 2.5), Complex::new(0.0, -7.0), Complex::new(6.0, 0.0), Complex::new(0.0, -4.0)]);
        let v3 = ComplexVector([Complex::new(22.0, -1.5), Complex::new(7.0, -4.0), Complex::new(10.2, -8.1), Complex::new(0.0, -7.0)]);
        assert_eq!(v1 + v2, v3);
    }

    #[test]
    fn test_vector_product_scalar() {
        let v1 = ComplexVector([Complex::new(6.0, 3.0), Complex::new(0.0, 0.0), Complex::new(5.0, 1.0), Complex::new(4.0, 0.0)]);
        let v2 = ComplexVector([Complex::new(12.0, 21.0), Complex::new(0.0, 0.0), Complex::new(13.0, 13.0), Complex::new(12.0, 8.0)]);

        assert_eq!(v1 * Complex::new(3.0, 2.0), v2);
    }

    #[test]
    fn test_vector_inverse() {
        let v1 = ComplexVector([Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
        let v2 = ComplexVector([Complex::new(-6.0, 4.0), Complex::new(-7.0, -3.0), Complex::new(-4.2, 8.1), Complex::new(0.0, 3.0)]);

        assert_eq!(-v1, v2);
    }

    #[test]
    fn test_inner_product() {
        let v1 = ComplexVector([Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
        let v2 = ComplexVector([Complex::new(16.0, 2.5), Complex::new(0.0, -7.0), Complex::new(6.0, 0.0), Complex::new(0.0, -4.0)]);

        assert_eq!(v1 * v2, Complex::new(102.2, 78.6));
    }

    #[test]
    fn test_distance() {
        let v1 = ComplexVector([Complex::new(3.0, 0.0), Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)]);
        let v2 = ComplexVector([Complex::new(2.0, 0.0), Complex::new(2.0, 0.0), Complex::new(-1.0, 0.0)]);

        assert_eq!(v1.distance_to(v2), v2.distance_to(v1));
        assert_eq!(v1.distance_to(v2), 11f64.sqrt());
    }
}
