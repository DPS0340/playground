#![allow(dead_code)]

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut res: [[i32; 3]; 3] = [[0; 3]; 3];

    matrix.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, _)| {
            let (j2, i2) = (i, j);
            res[i][j] = matrix[i2][j2];
        });
    });

    res
}

pub fn pretty_print(matrix: &[[i32; 3]; 3]) {
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
