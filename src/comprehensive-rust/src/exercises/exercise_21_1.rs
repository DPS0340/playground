// TODO: remove this when you're done with your implementation.
#![allow(unused_variables, dead_code)]

pub fn luhn(cc_number: &str) -> bool {
    cc_number
        .split(" ")
        .map(|e| e.to_string())
        .reduce(|a, b| a + &b)
        .filter(|e| e.len() >= 2)
        .filter(|e| e.parse::<u64>().is_ok())
        .map(|e| {
            e.as_bytes()
                .iter()
                .map(|e| *e as char)
                .rev()
                .map(|e| e.to_string().parse::<u64>().unwrap())
                .enumerate()
                .map(|(i, mut digit)| {
                    if i % 2 == 1 {
                        digit *= 2;
                    }
                    if digit >= 10 {
                        digit = (digit % 10) + (digit / 10);
                    }
                    digit
                })
                .sum::<u64>()
        })
        .filter(|e| (e % 10) == 0)
        .is_some()
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("  "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}
