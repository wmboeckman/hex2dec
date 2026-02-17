use std::fmt;

#[derive(Debug)]
pub enum ConversionErrors {
    IntConversionError,
    InvalidCharError,
    InvalidBasePrefixError,
    EmptyInputError,
    UndefinedBaseContextError,
}

impl fmt::Display for ConversionErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            ConversionErrors::IntConversionError => "failed to convert input to unsigned integer",
            ConversionErrors::InvalidCharError => "invalid character provided for conversion",
            ConversionErrors::InvalidBasePrefixError => "invalid base prefix in provided string",
            ConversionErrors::EmptyInputError => "provided string was empty",
            ConversionErrors::UndefinedBaseContextError => "provided base for conversion is undefined",
        };

        write!(f, "{}", msg)
    }
} 