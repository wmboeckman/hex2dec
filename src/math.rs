use std::fmt;

type Result<T> = std::result::Result<T, InvalidCharError>;

#[derive(Debug, Clone)]
pub struct InvalidCharError;

impl fmt::Display for InvalidCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid character for conversion in provided string")
    }
}

pub fn hex2dec(data: &String) -> Result<usize> {
    // 0x0000
    // 0x00000000
    // 0x[n times]

    // ...000[0,1,2,3,...,8,9,a,b,c,d,e,f] <-- len? 16!
    // 0x000f <-- 15
    // 0x001f <-- 31
    // 0x002f <-- 47
    // ...
    // 0x00ff <-- 255
    // 0x0fff <-- 4095 or (16^3)-1

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
             _  => {return Err(InvalidCharError)}
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