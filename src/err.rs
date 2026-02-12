use std::fmt;

pub enum ConversionErrors {
    InvalidCharError,
    InvalidBasePrefixError,
}

impl fmt::Display for ConversionErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            ConversionErrors::InvalidCharError => "invalid character provided for conversion",
            ConversionErrors::InvalidBasePrefixError => "invalid base prefix in provided string",
        };

        write!(f, "{}", msg)
    }
} 