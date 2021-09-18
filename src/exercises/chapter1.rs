use std::f64::consts::PI;

use crate::utils::complex_number::{Complex, Cartesian, Polar};

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

pub fn programming_drill_1_3_1() {
    println!("Solution to the programming drill 1.3.1.");

    let cartesian1 = Cartesian(1.0, 1.0);
    let cartesian2 = Cartesian(-1.0, 1.0);
    let polar1 = Polar(f64::sqrt(2.0), PI / 4.0);
    let polar2 = Polar(f64::sqrt(2.0), (3.0 * PI) / 4.0);

    println!("Cartesian {} to polar {}", cartesian1, Polar::from(cartesian1));
    println!("Cartesian {} to polar {}", cartesian2, Polar::from(cartesian2));
    println!("Polar {} to cartesian {}", polar1, Cartesian::from(polar1));
    println!("Polar {} to cartesian {}", polar2, Cartesian::from(polar2));
}

pub fn programming_drill_1_3_2() {
    println!("Solution to the programming drill 1.3.2.");
    let mut matrix = vec![vec![false; 15]; 15];

    // Drawing four tiles.
    matrix[5][7] = true;
    matrix[7][5] = true;
    matrix[7][9] = true;
    matrix[9][7] = true;

    // Print the first image.
    print_matrix(&matrix);
    println!("*****************************************");

    let mut new_matrix = vec![vec![false; 15]; 15];

    // Resizing and rotating factor.
    let factor = Complex::from(Polar(2.0, PI / 4.0));

    // Reposition tiles.
    for (y, row) in matrix.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            // We ignore blank elements because they could overlap (due to
            // rounding errors) with filled tiles. A more robust algorithm
            // would effectively prevent this, but that is out of the scope
            // of this exercise.
            if !(*elem) {
                continue;
            }

            // Compute the new position.
            let mut c = Complex::new((x as f64) - 7.0, (y as f64) - 7.0);
            c = c * factor;
            let Complex { real: r, imaginary: i } = c;
            let new_x = (r as i64) + 7;
            let new_y = (i as i64) + 7;

            // Ignore out of bounds.
            if new_x >= 0 && new_x < (row.len() as i64) {
                if new_y >= 0 && new_y < (matrix.len() as i64) {
                    new_matrix[(new_y as usize)][(new_x as usize)] = *elem;
                }
            }
        }
    }

    // Print the new image.
    print_matrix(&new_matrix);
}

fn print_matrix(matrix: &[Vec<bool>]) {
    matrix.iter().enumerate().for_each(|(i, row)| {
        print!("{}\t", i);
        row.iter().enumerate().for_each(|(_, col)| {
            print!("{}", if *col { "■" } else { "□" });
        });
        println!()
    });
}
