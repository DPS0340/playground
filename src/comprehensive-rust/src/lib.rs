mod exercise_7_1;
mod exercise_7_2;

#[cfg(test)]
mod tests {
    use crate::exercise_7_1::*;
    use crate::exercise_7_2::*;

    #[test]
    fn test_7_1() {
        let x: i8 = 15;
        let y: i16 = 1000;

        assert_eq!(i16::from(x) * y, multiply(x.into(), y));
    }

    #[test]
    fn test_7_2() {
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
}
