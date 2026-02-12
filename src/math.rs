use crate::err::ConversionErrors;

pub fn hex2dec(data: &String) -> Result<usize, ConversionErrors> {    

    // TODO: check input for valid base prefix!

    let mut pos: u32 = 0; // digit position tracker
    let mut counter: usize = 0;

    for c in data.to_lowercase().chars().rev() {
        match c {
            // TODO: map 0..f to 0..15 !

            '0' => {counter += mult_pos(pos) * 0 },
            '1' => {counter += mult_pos(pos) * 1 },
            '2' => {counter += mult_pos(pos) * 2 },
            '3' => {counter += mult_pos(pos) * 3 },
            '4' => {counter += mult_pos(pos) * 4 },
            '5' => {counter += mult_pos(pos) * 5 },
            '6' => {counter += mult_pos(pos) * 6 },
            '7' => {counter += mult_pos(pos) * 7 },
            '8' => {counter += mult_pos(pos) * 8 },
            '9' => {counter += mult_pos(pos) * 9 },
            'a' => {counter += mult_pos(pos) * 10 },
            'b' => {counter += mult_pos(pos) * 11 },
            'c' => {counter += mult_pos(pos) * 12 },
            'd' => {counter += mult_pos(pos) * 13 },
            'e' => {counter += mult_pos(pos) * 14 },
            'f' => {counter += mult_pos(pos) * 15 },
             _  => {return Err(ConversionErrors::InvalidCharError)}
        }

        // println!("{}: {} : {}", c, counter, mult_pos(pos));

        pos += 1;
    }
    return Ok(counter);
}

fn mult_pos(p: u32) -> usize {
    return usize::pow(16, p);
}

pub fn dec2hex(data: &usize) -> String {
    let mut result = String::new();
    
    // do stuff
    
    return result;
}