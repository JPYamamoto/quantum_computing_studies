use std::ops::{Add, Mul, Neg, Index};
use std::fmt::Display;

use crate::utils::complex_number::Complex;

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
}
