use std::fmt;

#[derive(Debug)]
pub enum ConversionErrors {
    IntConversionError,
    InvalidCharError,
    InvalidBasePrefixError,
    EmptyInputError,
    UndefinedBaseContextError,
    LargeInputError
}

impl fmt::Display for ConversionErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match &self {
            ConversionErrors::IntConversionError => "failed to convert input to unsigned integer",
            ConversionErrors::InvalidCharError => "invalid character provided for conversion",
            ConversionErrors::InvalidBasePrefixError => "invalid base prefix in provided string",
            ConversionErrors::EmptyInputError => "provided string was empty",
            ConversionErrors::UndefinedBaseContextError => "provided base for conversion is undefined",
            ConversionErrors::LargeInputError => "provided input was too large to process"
        };

        write!(f, "{}", msg)
    }
} 