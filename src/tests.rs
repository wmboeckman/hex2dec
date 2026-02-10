#[cfg(test)]
use crate::math::*;

#[test]
fn test_hex2dec() {
    // edge-cases
    assert_eq!(hex2dec(&String::from("0")), 0 as u32);
    assert_eq!(hex2dec(&String::from("0000")), 0 as u32);
    assert_eq!(hex2dec(&String::from("000f")), 15 as u32);
    assert_eq!(hex2dec(&String::from("00f0")), 240 as u32);
    assert_eq!(hex2dec(&String::from("0f00")), 3840 as u32);
    assert_eq!(hex2dec(&String::from("f000")), 61440 as u32);

    // ...
}