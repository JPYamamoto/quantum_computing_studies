use crate::utils::complex_number::Complex;
use crate::utils::complex_vector::ComplexVector;
use crate::utils::complex_matrix::ComplexMatrix;

pub fn programming_drill_2_1_1() {
    println!("Solution to the programming drill 2.1.1.");

    let v1 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
    print!("-{} = ", v1);
    println!("{}", -v1);

    let v2 = ComplexVector(vec![Complex::new(6.0, 3.0), Complex::new(0.0, 0.0), Complex::new(5.0, 1.0), Complex::new(4.0, 0.0)]);
    print!("{} * {} = ", Complex::new(3.0, 2.0), v2);
    println!("{}", v2 * Complex::new(3.0, 2.0));

    let v3 = ComplexVector(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)]);
    let v4 = ComplexVector(vec![Complex::new(16.0, 2.5), Complex::new(0.0, -7.0), Complex::new(6.0, 0.0), Complex::new(0.0, -4.0)]);
    print!("{} + {} = ", v3, v4);
    println!("{}", v3+v4);
}

pub fn programming_drill_2_2_1() {
    println!("Solution to the programming drill 2.2.1.");

    let v1 = ComplexMatrix::new(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)], 2, 2);
    print!("-{} = ", v1);
    println!("{}", -v1);

    let v2 = ComplexMatrix::new(vec![Complex::new(6.0, 3.0), Complex::new(0.0, 0.0), Complex::new(5.0, 1.0), Complex::new(4.0, 0.0)], 2, 2);
    print!("{} * {} = ", Complex::new(3.0, 2.0), v2);
    println!("{}", v2 * Complex::new(3.0, 2.0));

    let v3 = ComplexMatrix::new(vec![Complex::new(6.0, -4.0), Complex::new(7.0, 3.0), Complex::new(4.2, -8.1), Complex::new(0.0, -3.0)], 2, 2);
    let v4 = ComplexMatrix::new(vec![Complex::new(16.0, 2.5), Complex::new(0.0, -7.0), Complex::new(6.0, 0.0), Complex::new(0.0, -4.0)], 2, 2);
    print!("{} + {} = ", v3, v4);
    println!("{}", v3+v4);
}
