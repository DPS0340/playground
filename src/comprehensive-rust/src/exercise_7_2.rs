use core::fmt;

fn transpose<T: Default + Copy, const R: usize, const C: usize>(
    matrix: [[T; R]; C],
) -> [[T; C]; R] {
    let mut res = [[T::default(); C]; R];

    for j in 0..C {
        for i in 0..R {
            res[i][j] = matrix[j][i];
        }
    }

    res
}

fn pretty_print<T: fmt::Display, const R: usize, const C: usize>(matrix: &[[T; R]; C]) {
    print!("[");
    matrix.iter().enumerate().for_each(|(i, row)| {
        print!("[");
        row.iter().enumerate().for_each(|(i, e)| {
            print!("{}", e);
            if i != row.len() - 1 {
                print!(", ");
            }
        });
        print!("]");
        if i != matrix.len() - 1 {
            println!();
        }
    });
    println!("]");
}

#[test]
fn test() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
        [401, 402, 403],
    ];
    let transposed_answer = [
        [101, 201, 301, 401], // same
        [102, 202, 302, 402],
        [103, 203, 303, 403],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);

    assert_eq!(transposed, transposed_answer);
}
