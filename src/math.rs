use crate::err::ConversionErrors;

pub fn hex2dec(data: &String, debug: bool) -> Result<String, ConversionErrors> {    

    let ordered_chars = "0123456789abcdef";

    let mut pos: u32 = 0; // digit position tracker
    let mut counter: usize = 0;

    for c in data.to_lowercase().chars().rev() {

        counter += usize::pow(16, pos) * ordered_chars.find(c).ok_or(ConversionErrors::InvalidCharError)?;

        if debug {
            println!("pos: {}:{}\tcounter: {}", pos, c, counter);
        }

        pos += 1;
    }
    return Ok(counter.to_string());
}

pub fn dec2hex(data: &String, debug: bool) -> Result<String, ConversionErrors> {
    
    let mut result = String::new();
    result.push_str("0x");

    let mut number = match data.parse::<usize>() {
        Ok(i) => i,
        Err(_e,) => return Err(ConversionErrors::IntConversionError),
    };
    
    // Edge case for 0
    if number == 0 {
        return Ok(String::from("0x0"));
    }

    // A lookup table to map remainders to hex characters
    let ordered_chars = "0123456789abcdef".as_bytes();

    ordered_chars[0];

    let mut result = String::new();

    while number > 0 {
        // Get the last hex digit
        let remainder = (number % 16) as usize;
        // Push corresponding char to string
        result.push(ordered_chars[remainder] as char);
        // Move to the next hex digit
        number /= 16;
    }

    // Because we pushed remainders from right-to-left, we must reverse the string
    return Ok(result.chars().rev().collect());
} 