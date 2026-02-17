// use crate::{err::ConversionErrors, util::linear_search};
use crate::util::{linear_search, err::*};


pub fn dec2base(
    data: &usize,
    base: usize,
    charset: &[char],
    prefix: &[char; 2],
) -> Result<String, ConversionErrors> {
    let mut result = String::new();

    let mut counter = data.clone();

    // Edge case for 0: print 0-value char from charset
    if counter == 0 {
        result.push(charset[0]);
    }

    while counter > 0 {
        // last hex digit's value offset
        let remainder = (counter % base) as usize;
        result.push(charset[remainder] as char);
        // Move to the next hex digit
        counter /= base;
    }

    // reverse-printed prefix
    result.push(prefix[1] as char);
    result.push(prefix[0] as char);

    // Because we pushed remainders in reverse, we must un-reverse
    return Ok(result.chars().rev().collect());
}

pub fn base2dec(
    data: &String,
    base: usize,
    charset: &[char],
    prefix: &[char; 2],
) -> Result<usize, ConversionErrors> {
    let mut pf = String::new();
    pf.push(prefix[0]);
    pf.push(prefix[1]);

    if data.is_empty() { return Err(ConversionErrors::EmptyInputError); }

    // check and strip the prefix
    if data.as_bytes()[0] as char != prefix[0] || data.as_bytes()[1] as char != prefix[1] {
        return Err(ConversionErrors::InvalidBasePrefixError);
    }

    let stripped_data = &data[2..];

    let mut pos: u16 = 0;
    let mut counter: usize = 0;

    // iterate over each char in reverse
    for c in stripped_data.to_ascii_lowercase().chars().rev() {
        // attempt to find index within charset

        let index = match linear_search(&mut charset.iter(), &c) {
            Some(i) => i,
            None => {
                return Err(ConversionErrors::InvalidCharError);
            }
        };

        counter += usize::pow(base, pos as u32) * index;

        // if debug {
        //     println!("pos: {}:{}\tcounter: {}", pos, c, counter);
        // }

        pos += 1;
    }
    return Ok(counter);
}
