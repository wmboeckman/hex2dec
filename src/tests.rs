#[cfg(test)]
use crate::math::*;

#[test]
fn test_basic_conversion() {
    // Simple single digits
    assert_eq!(hex2dec(&String::from("1"), false).unwrap(), "1");
    assert_eq!(hex2dec(&String::from("9"), false).unwrap(), "9");
    assert_eq!(hex2dec(&String::from("A"), false).unwrap(), "10");
    assert_eq!(hex2dec(&String::from("F"), false).unwrap(), "15");
}

#[test]
fn test_multi_digit_conversion() {
    // Testing positional weight (16^1, 16^0, etc.)
    assert_eq!(hex2dec(&String::from("10"),   false).unwrap(), "16");
    assert_eq!(hex2dec(&String::from("FF"),   false).unwrap(), "255");
    assert_eq!(hex2dec(&String::from("1A2B"), false).unwrap(), "6699");

    assert_eq!(hex2dec(&String::from("0"),    false).unwrap(), "0");
    assert_eq!(hex2dec(&String::from("0000"), false).unwrap(), "0");
    assert_eq!(hex2dec(&String::from("000f"), false).unwrap(), "15");
    assert_eq!(hex2dec(&String::from("00f0"), false).unwrap(), "240");
    assert_eq!(hex2dec(&String::from("0f00"), false).unwrap(), "3840");
    assert_eq!(hex2dec(&String::from("f000"), false).unwrap(), "61440");
}

#[test]
fn test_case_insensitivity() {
    // Users might input uppercase or lowercase
    assert_eq!(hex2dec(&String::from("abc"),  false).unwrap(), hex2dec(&String::from("ABC"),  false).unwrap());
    assert_eq!(hex2dec(&String::from("a1b2"), false).unwrap(), hex2dec(&String::from("A1B2"), false).unwrap());
}

#[test]
fn test_large_values() {
    // Testing the upper bounds of a 64-bit integer
    assert_eq!(hex2dec(&String::from("FFFFFFFF"), false).unwrap(), "4294967295");
}

#[test]
fn test_invalid_characters() {
    // The function should return an Error for non-hex inputs
    let result = hex2dec(&String::from("12G4"), false);
    assert!(result.is_err());
}

#[test]
fn test_empty_string() {
    // Deciding how to handle an empty input
    let result = hex2dec(&String::from(""), false).unwrap();
    assert!(result == "0");
}