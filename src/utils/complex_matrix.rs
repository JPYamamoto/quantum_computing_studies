use std::ops::{Add, Mul, Neg, Index, IndexMut};
use std::fmt::Display;

use crate::utils::complex_number::Complex;
use crate::utils::complex_vector::ComplexVector;

#[derive(Debug, PartialEq)]
pub struct ComplexMatrix{
    elements: Vec<Complex>,
    rows: usize,
    columns: usize,
}

impl ComplexMatrix {
    pub fn new(vector: Vec<Complex>, rows: usize, columns: usize) -> Self {
        if vector.len() != (rows * columns) {
            panic!("Vector should be of size rows * columns.")
        }

        ComplexMatrix { elements: vector, rows, columns }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.rows, self.columns)
    }
}

impl From<ComplexVector> for ComplexMatrix {
    fn from(ComplexVector(rhs): ComplexVector) -> Self {
        let rows = rhs.len();
        ComplexMatrix { elements: rhs, rows, columns: 1 }
    }
}

impl Index<[usize; 2]> for ComplexMatrix {
    type Output = Complex;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let [row, column] = &index;

        if row >= &self.rows || column >= &self.columns {
            panic!("Index out of range.")
        }

        &self.elements[(row * &self.columns) + column]
    }
}

impl IndexMut<[usize; 2]> for ComplexMatrix {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let [row, column] = &index;

        if row >= &self.rows || column >= &self.columns {
            panic!("Index out of range.")
        }

        &mut self.elements[(row * &self.columns) + column]
    }


}

impl Add for ComplexMatrix {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        add_matrices(self, rhs)
    }
}

/// Support for scalar product on complex matrices.
impl Mul<Complex> for ComplexMatrix {
    type Output = Self;

    fn mul(self, rhs: Complex) -> Self::Output {
        product_matrix_scalar(self, rhs)
    }
}

/// Support for vector-matrix product.
impl Mul<ComplexVector> for ComplexMatrix {
    type Output = Self;

    fn mul(self, rhs: ComplexVector) -> Self::Output {
        product_matrices(self, ComplexMatrix::from(rhs))
    }
}

/// Support for product on complex matrices.
impl Mul<ComplexMatrix> for ComplexMatrix {
    type Output = Self;

    fn mul(self, rhs: ComplexMatrix) -> Self::Output {
        product_matrices(self, rhs)
    }
}

/// Support for negating complex matrices.
impl Neg for ComplexMatrix {
    type Output = Self;

    fn neg(self) -> Self::Output {
        inverse_matrix(self)
    }
}

/// Support for displaying complex matrices.
impl Display for ComplexMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result_string = String::new();

        for r in 0..self.rows {
            let mut row_display = String::new();

            for c in 0..self.columns {
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
fn add_matrices(matrix1: ComplexMatrix, matrix2: ComplexMatrix) -> ComplexMatrix {
    if matrix1.dimensions() != matrix2.dimensions() {
        panic!("Cannot add matrices of different dimensions.");
    }

    let result_vector: Vec<Complex> = matrix1.elements
                                             .iter()
                                             .zip(matrix2.elements.iter())
                                             .map(|(&x,&y)| x + y)
                                             .collect();

    ComplexMatrix {
        elements: result_vector,
        rows: matrix1.rows,
        columns: matrix1.columns
    }
}

/// Coordinate-wise complex scalar by complex matrix product.
fn product_matrix_scalar(matrix: ComplexMatrix, scalar: Complex) -> ComplexMatrix {
    let new_elements = matrix.elements.iter().map(|&x| x * scalar).collect();

    ComplexMatrix {
        elements: new_elements,
        rows: matrix.rows,
        columns: matrix.columns
    }
}

/// Matrix-Vector product.
pub fn product_matrix_vector(matrix: ComplexMatrix, vector: ComplexVector) -> ComplexVector {
    //let vec_to_mat = ComplexMatrix::from(vector);
    //let ComplexMatrix { elements, .. } = matrix * vec_to_mat;
    //ComplexVector(elements)
    //
    let vec_to_mat = ComplexMatrix::from(vector);
    let ComplexMatrix { elements, .. } = matrix * vec_to_mat;
    ComplexVector(elements)
}

/// Standard complex matrices product.
fn product_matrices(m1: ComplexMatrix, m2: ComplexMatrix) -> ComplexMatrix {
    if m1.columns != m2.rows {
        panic!("Number of columns in the left-hand side matrix should be the \
                same as number of rows in the right-hand side matrix.");
    }

    let mut m3 = ComplexMatrix::new(
        vec![Complex::new(0.0, 0.0); m1.rows * m2.columns], m1.rows, m2.columns);

    for j in 0..m3.rows {
        for k in 0..m3.columns {
            let mut sum = Complex::new(0.0, 0.0);

            for h in 0..m1.columns {
                sum += m1[[j,h]] * m2[[h,k]]
            }

            m3[[j,k]] = sum;
        }
    }

    m3
}

/// Inverse over addition matrix, by negating each coordinate.
fn inverse_matrix(matrix: ComplexMatrix) -> ComplexMatrix {
    ComplexMatrix {
        elements: matrix.elements.iter().map(|&x| -x).collect(),
        rows: matrix.rows,
        columns: matrix.columns
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_matrix() {
        let v = ComplexVector(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)]);
        let m = ComplexMatrix::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)], 4, 1);
        assert_eq!(ComplexMatrix::from(v), m);
    }

    #[test]
    fn test_matrix_product_vector() {
        let m = ComplexMatrix::new(vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0), Complex::new(3.0, 0.0), Complex::new(4.0, 0.0)], 2, 2);
        let v1 = ComplexVector(vec![Complex::new(1.0, 0.0), Complex::new(2.0, 0.0)]);
        let v2 = ComplexVector(vec![Complex::new(5.0, 0.0), Complex::new(11.0, 0.0)]);
        assert_eq!(product_matrix_vector(m, v1), v2);
    }

    #[test]
    fn test_matrix_add() {
        let m1 = ComplexMatrix::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)], 2, 2);
        let m2 = ComplexMatrix::new(vec![Complex::new(1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(1.0, 0.0)], 2, 2);
        let m3 = ComplexMatrix::new(vec![Complex::new(2.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(2.0, 0.0)], 2, 2);
        assert_eq!(m1 + m2, m3);
    }

    #[test]
    fn test_matrix_product_scalar() {
        let m1 = ComplexMatrix::new(vec![Complex::new(0.0, 1.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 1.0)], 2, 2);
        let m2 = ComplexMatrix::new(vec![Complex::new(-1.0, 0.0), Complex::new(0.0, 0.0), Complex::new(0.0, 0.0), Complex::new(-1.0, 0.0)], 2, 2);

        assert_eq!(m1 * Complex::new(0.0, 1.0), m2);
    }

    #[test]
    fn test_matrix_inverse() {
        let m1 = ComplexMatrix::new(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)], 2, 2);
        let m2 = ComplexMatrix::new(vec![Complex::new(-6.0, 4.0), Complex::new(-7.0, -3.0), Complex::new(-4.2, 8.1), Complex::new(0.0, 3.0)], 2, 2);

        assert_eq!(-m1, m2);
    }

    #[test]
    fn test_matrix_product() {
        let m1 = ComplexMatrix::new(vec![Complex::new(3.0, 2.0), Complex::new(0.0, 0.0), Complex::new(5.0, -6.0),
                                         Complex::new(1.0, 0.0), Complex::new(4.0, 2.0), Complex::new(0.0, 1.0),
                                         Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0, 0.0)], 3, 3);
        let m2 = ComplexMatrix::new(vec![Complex::new(5.0, 0.0), Complex::new(2.0, -1.0), Complex::new(6.0, -4.0),
                                         Complex::new(0.0, 0.0), Complex::new(4.0, 5.0), Complex::new(2.0, 0.0),
                                         Complex::new(7.0, -4.0), Complex::new(2.0, 7.0), Complex::new(0.0, 0.0)], 3, 3);
        let m3 = ComplexMatrix::new(vec![Complex::new(26.0, -52.0), Complex::new(60.0, 24.0), Complex::new(26.0, 0.0),
                                         Complex::new(9.0, 7.0), Complex::new(1.0, 29.0), Complex::new(14.0, 0.0),
                                         Complex::new(48.0, -21.0), Complex::new(15.0, 22.0), Complex::new(20.0, -22.0)], 3, 3);

        assert_eq!(m1 * m2, m3);
    }

    #[test]
    #[should_panic]
    #[allow(unused_must_use)]
    fn test_matrix_product_error() {
        let m1 = ComplexMatrix::new(vec![Complex::new(3.0, 2.0), Complex::new(0.0, 0.0), Complex::new(5.0, -6.0),
                                         Complex::new(1.0, 0.0), Complex::new(4.0, 2.0), Complex::new(0.0, 1.0),
                                         Complex::new(4.0, -1.0), Complex::new(0.0, 0.0), Complex::new(4.0, 0.0)], 3, 3);
        let m2 = ComplexMatrix::new(vec![Complex::new(5.0, 0.0), Complex::new(2.0, -1.0),
                                         Complex::new(0.0, 0.0), Complex::new(4.0, 5.0)], 2, 2);

        m1 * m2;
    }
}
