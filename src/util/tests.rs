use crate::math::conversion::*;
use crate::math::context::*;

#[test]
fn test_basic_conversion() {
    // 
    const MAX_DECIMAL: usize = 10000;

    for context in CONTEXT_REGISTRY.contexts {
        for n in 0..MAX_DECIMAL {
            let input = match context.base {
                16 => format!("{:#x}", n).to_string(),
                8 => format!("{:#o}", n).to_string(),
                _ => panic!()
            };

            assert_eq!(base2dec(&input, context.base, &context.charset, &context.prefix).unwrap(), n);
        }
    }
}

#[test]
fn test_case_insensitivity() {
    // Users might input uppercase or lowercase
    assert_eq!(base2dec(&"0xabc".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), base2dec(&"0xABC".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap());
    assert_eq!(base2dec(&"0xa1b2".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), base2dec(&"0xA1B2".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap());
}

#[test]
fn test_large_values() {
    // Testing the upper bounds of the 'usize' integer type
    assert_eq!(base2dec(&format!("{:#x}", std::usize::MAX).to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).unwrap(), std::usize::MAX);
    assert_eq!(base2dec(&format!("{:#o}", std::usize::MAX).to_string(), CONTEXT_B8.base, &CONTEXT_B8.charset, &CONTEXT_B8.prefix).unwrap(), std::usize::MAX);
}

#[test]
fn test_invalid_characters() {
    // Should return an Error for invalid chars
    assert!(base2dec(&"0x12G4".to_string(), CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix).is_err());
    assert!(base2dec(&"0o1289".to_string(), CONTEXT_B8.base, &CONTEXT_B8.charset, &CONTEXT_B8.prefix).is_err());
}

#[test]
fn test_empty_string() {
    // Empty input returns error
    for context in CONTEXT_REGISTRY.contexts {
        assert!(base2dec(&"".to_string(), context.base, &context.charset, &context.prefix).is_err());
    }
}