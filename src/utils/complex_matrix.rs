use std::ops::{Add, Mul, Neg, Index, IndexMut};
use std::fmt::Display;

use crate::utils::complex_number::Complex;
use crate::utils::complex_vector::ComplexVector;

#[derive(Debug, PartialEq)]
pub struct ComplexMatrix<const R: usize, const C: usize>([[Complex; C]; R]);

impl<const R: usize, const C: usize> ComplexMatrix<R, C> {
    pub fn new(values: [[Complex; C]; R]) -> Self {
        ComplexMatrix(values)
    }
}

impl<const N: usize> From<ComplexVector<N>> for ComplexMatrix<N, 1> {
    fn from(ComplexVector(rhs): ComplexVector<N>) -> Self {
        ComplexMatrix(rhs.map(|c| [c]))
    }
}

impl<const R: usize, const C: usize> Index<[usize; 2]> for ComplexMatrix<R, C> {
    type Output = Complex;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, column] = index;

        if row >= R || column >= C {
            panic!("Index out of range.")
        }

        &self.0[row][column]
    }
}

impl<const R: usize, const C: usize> IndexMut<[usize; 2]> for ComplexMatrix<R, C> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, column] = index;

        if row >= R || column >= C {
            panic!("Index out of range.")
        }

        &mut self.0[row][column]
    }


}

impl<const R: usize, const C: usize> Add for ComplexMatrix<R, C> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add_matrices(self, rhs)
    }
}

/// Support for scalar product on complex matrices.
impl<const R: usize, const C: usize> Mul<Complex> for ComplexMatrix<R, C> {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        product_matrix_scalar(self, rhs)
    }
}

/// Support for vector-matrix product.
impl<const R: usize, const C: usize> Mul<ComplexVector<C>> for ComplexMatrix<R, C> {
    type Output = ComplexMatrix<R, 1>;

    fn mul(self, rhs: ComplexVector<C>) -> Self::Output {
        product_matrices(self, ComplexMatrix::from(rhs))
    }
}

/// Support for product on complex matrices.
impl<const R: usize, const C: usize, const P: usize> Mul<ComplexMatrix<C, P>> for ComplexMatrix<R, C> {
    type Output = ComplexMatrix<R, P>;

    fn mul(self, rhs: ComplexMatrix<C, P>) -> Self::Output {
        product_matrices(self, rhs)
    }
}

/// Support for negating complex matrices.
impl<const R: usize, const C: usize> Neg for ComplexMatrix<R, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        negated_matrix(self)
    }
}

/// Support for displaying complex matrices.
impl<const R: usize, const C: usize> Display for ComplexMatrix<R, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result_string = String::new();

        for r in 0..R {
            let mut row_display = String::new();

            for c in 0..C {
                row_display.push_str(format!("{}, ", self[[r, c]]).as_str());
            }

            row_display = row_display[0..row_display.len() - 2].to_string();
            result_string.push_str(format!("[{}],", row_display).as_str());
        }

        // Remove the last char, standing for the last separator ","
        result_string.pop();
        write!(f, "[{}]", &result_string)
    }
}

/// Coordinate-wise matrix addition.
fn add_matrices<const R: usize, const C: usize>(matrix1: ComplexMatrix<R, C>, matrix2: ComplexMatrix<R, C>) -> ComplexMatrix<R, C> {
    let mut result_array: [[Complex; C]; R] = [[Complex::new(0.0, 0.0); C]; R];

    for y in 0..C {
        for x in 0..R {
            result_array[x][y] = matrix1[[x, y]] + matrix2[[x, y]];
        }
    }

    ComplexMatrix(result_array)
}

/// Coordinate-wise complex scalar by complex matrix product.
fn product_matrix_scalar<const R: usize, const C: usize>(matrix: ComplexMatrix<R, C>, scalar: Complex) -> ComplexMatrix<R, C> {
    let new_elements = matrix.0.map(|arr| arr.map(|x| scalar * x));

    ComplexMatrix(new_elements)
}

/// Matrix-Vector product.
pub fn product_matrix_vector<const R: usize, const C: usize>(matrix: ComplexMatrix<R, C>, vector: ComplexVector<C>) -> ComplexVector<R> {
    let vec_to_mat = ComplexMatrix::from(vector);
    let result_matrix = matrix * vec_to_mat;
    let result_vector = result_matrix.0.map(|row| row[0]);
    ComplexVector(result_vector)
}

/// Standard complex matrices product.
fn product_matrices<const R: usize, const C: usize, const P: usize>(m1: ComplexMatrix<R, C>, m2: ComplexMatrix<C, P>) -> ComplexMatrix<R, P> {
    let mut m3 = ComplexMatrix::new([[Complex::new(0.0, 0.0); P]; R]);

    for j in 0..R {
        for k in 0..P {
            let mut sum = Complex::new(0.0, 0.0);

            for h in 0..C {
                sum += m1[[j,h]] * m2[[h,k]]
            }

            m3[[j,k]] = sum;
        }
    }

    m3
}

/// Inverse over addition matrix, by negating each coordinate.
fn negated_matrix<const R: usize, const C: usize>(matrix: ComplexMatrix<R, C>) -> ComplexMatrix<R, C> {
    ComplexMatrix(matrix.0.map(|row| row.map(|x| -x)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_matrix() {
        let v = ComplexVector([Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);
        let m = ComplexMatrix::new([[Complex::new(1.0, 0.0)], [Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0)], [Complex::new(1.0, 0.0)]]);
        assert_eq!(ComplexMatrix::from(v), m);
    }

    #[test]
    fn test_matrix_product_vector() {
        let m = ComplexMatrix::new([[Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)], [Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)]]);
        let v1 = ComplexVector([Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)]);
        let v2 = ComplexVector([Complex::new(5.0, 0.0), Complex::new(11.0, 0.0)]);
        assert_eq!(product_matrix_vector(m, v1), v2);
    }

    #[test]
    fn test_matrix_add() {
        let m1 = ComplexMatrix::new([[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]]);
        let m2 = ComplexMatrix::new([[Complex::new(1.0, 0.0), Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]]);
        let m3 = ComplexMatrix::new([[Complex::new(2.0, 0.0), Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0), Complex::new(2.0, 0.0)]]);
        assert_eq!(m1 + m2, m3);
    }

    #[test]
    fn test_matrix_product_scalar() {
        let m1 = ComplexMatrix::new([[Complex::new(0.0, 1.0), Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)]]);
        let m2 = ComplexMatrix::new([[Complex::new(-1.0, 0.0), Complex::new(0.0, 0.0)], [Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)]]);

        assert_eq!(m1 * Complex::new(0.0, 1.0), m2);
    }

    #[test]
    fn test_matrix_inverse() {
        let m1 = ComplexMatrix::new([[Complex::new(6.0, -4.0), Complex::new(7.0, 3.0)], [Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]]);
        let m2 = ComplexMatrix::new([[Complex::new(-6.0, 4.0), Complex::new(-7.0, -3.0)], [Complex::new(-4.2, 8.1), Complex::new(0.0, 3.0)]]);

        assert_eq!(-m1, m2);
    }

    #[test]
    fn test_matrix_product() {
        let m1 = ComplexMatrix::new([[Complex::new(3.0, 2.0), Complex::new(0.0, 0.0), Complex::new(5.0, -6.0)],
                                     [Complex::new(1.0, 0.0), Complex::new(4.0, 2.0), Complex::new(0.0, 1.0)],
                                     [Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0, 0.0)]]);
        let m2 = ComplexMatrix::new([[Complex::new(5.0, 0.0), Complex::new(2.0, -1.0), Complex::new(6.0, -4.0)],
                                     [Complex::new(0.0, 0.0), Complex::new(4.0, 5.0), Complex::new(2.0, 0.0)],
                                     [Complex::new(7.0, -4.0), Complex::new(2.0, 7.0), Complex::new(0.0, 0.0)]]);
        let m3 = ComplexMatrix::new([[Complex::new(26.0, -52.0), Complex::new(60.0, 24.0), Complex::new(26.0, 0.0)],
                                     [Complex::new(9.0, 7.0), Complex::new(1.0, 29.0), Complex::new(14.0, 0.0)],
                                     [Complex::new(48.0, -21.0), Complex::new(15.0, 22.0), Complex::new(20.0, -22.0)]]);

        assert_eq!(m1 * m2, m3);
    }
}
