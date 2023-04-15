fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

#[test]
fn test() {
    let x: i8 = 15;
    let y: i16 = 1000;

    assert_eq!(i16::from(x) * y, multiply(x.into(), y));
}
