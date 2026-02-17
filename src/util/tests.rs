use crate::math::conversion::*;
use crate::math::context::*;

#[test]
fn test_basic_conversion() {
    // Simple single digits
    assert_eq!(base2dec(&"0x0".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 0);
    assert_eq!(base2dec(&"0x1".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 1);
    assert_eq!(base2dec(&"0x9".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 9);
    assert_eq!(base2dec(&"0xA".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 10);
    assert_eq!(base2dec(&"0xF".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 15);
}

#[test]
fn test_multi_digit_conversion() {
    // Testing positional weight (16^1, 16^0, etc.)
    assert_eq!(base2dec(&"0x0010".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 16);
    assert_eq!(base2dec(&"0x00FF".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 255);
    assert_eq!(base2dec(&"0x1A2B".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 6699);
    assert_eq!(base2dec(&"0x0000".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 0);
    assert_eq!(base2dec(&"0x000f".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 15);
    assert_eq!(base2dec(&"0x00f0".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 240);
    assert_eq!(base2dec(&"0x0f00".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 3840);
    assert_eq!(base2dec(&"0xf000".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 61440);
}

#[test]
fn test_case_insensitivity() {
    // Users might input uppercase or lowercase
    assert_eq!(base2dec(&"0xabc".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), base2dec(&"0xABC".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap());
    assert_eq!(base2dec(&"0xa1b2".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), base2dec(&"0xA1B2".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap());
}

#[test]
fn test_large_values() {
    // Testing the upper bounds of a 64-bit integer
    assert_eq!(base2dec(&"0xFFFFFFFF".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), 4294967295);
}

#[test]
fn test_invalid_characters() {
    // The function should return an Error for non-hex inputs
    assert!(base2dec(&"0x12G4".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).is_err());
}

#[test]
fn test_empty_string() {
    // Deciding how to handle an empty input
    assert!(base2dec(&"".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).is_err());
}