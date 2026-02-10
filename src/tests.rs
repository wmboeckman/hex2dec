#[cfg(test)]
use crate::math::*;

#[test]
fn test_basic_conversion() {
    // Simple single digits
    assert_eq!(hex2dec(&String::from("1")).unwrap(), 1);
    assert_eq!(hex2dec(&String::from("9")).unwrap(), 9);
    assert_eq!(hex2dec(&String::from("A")).unwrap(), 10);
    assert_eq!(hex2dec(&String::from("F")).unwrap(), 15);
}

#[test]
fn test_multi_digit_conversion() {
    // Testing positional weight (16^1, 16^0, etc.)
    assert_eq!(hex2dec(&String::from("10")).unwrap(), 16);
    assert_eq!(hex2dec(&String::from("FF")).unwrap(), 255);
    assert_eq!(hex2dec(&String::from("1A2B")).unwrap(), 6699);

    assert_eq!(hex2dec(&String::from("0")).unwrap(), 0);
    assert_eq!(hex2dec(&String::from("0000")).unwrap(), 0);
    assert_eq!(hex2dec(&String::from("000f")).unwrap(), 15);
    assert_eq!(hex2dec(&String::from("00f0")).unwrap(), 240);
    assert_eq!(hex2dec(&String::from("0f00")).unwrap(), 3840);
    assert_eq!(hex2dec(&String::from("f000")).unwrap(), 61440);
}

#[test]
fn test_case_insensitivity() {
    // Users might input uppercase or lowercase
    assert_eq!(hex2dec(&String::from("abc")).unwrap(), hex2dec(&String::from("ABC")).unwrap());
    assert_eq!(hex2dec(&String::from("a1b2")).unwrap(), hex2dec(&String::from("A1B2")).unwrap());
}

#[test]
fn test_large_values() {
    // Testing the upper bounds of a 64-bit integer
    assert_eq!(hex2dec(&String::from("FFFFFFFF")).unwrap(), 4294967295);
}

#[test]
fn test_invalid_characters() {
    // The function should return an Error for non-hex inputs
    let result = hex2dec(&String::from("12G4"));
    assert!(result.is_err());
}

#[test]
fn test_empty_string() {
    // Deciding how to handle an empty input
    let result = hex2dec(&String::from("")).unwrap();
    assert!(result == 0);
}