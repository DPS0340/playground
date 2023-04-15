#![allow(dead_code)]

use core::fmt;

pub fn transpose<T: Default + Copy, const R: usize, const C: usize>(
    matrix: [[T; R]; C],
) -> [[T; C]; R] {
    let mut res = [[T::default(); C]; R];

    (0..C).for_each(|j| {
        (0..R).for_each(|i| {
            let (j2, i2) = (i, j);
            res[i][j] = matrix[i2][j2];
        });
    });

    res
}

pub fn pretty_print<T: fmt::Display, const R: usize, const C: usize>(matrix: &[[T; R]; C]) {
    print!("{{");
    matrix.iter().enumerate().for_each(|(i, row)| {
        print!("{{");
        row.iter().enumerate().for_each(|(i, e)| {
            print!("{}", e);
            if i != row.len() - 1 {
                print!(", ");
            }
        });
        print!("}}");
        if i != matrix.len() - 1 {
            println!();
        }
    });
    println!("}}");
}
