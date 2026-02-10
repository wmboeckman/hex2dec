pub fn hex2dec(data: &String) -> u32 {
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

    let mut pos: u16 = 0; // digit position tracker
    let mut counter: u32 = 0;

    for c in data.chars().rev() {
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
             _  => {panic!("HOW DID YOU GET HERE");}
        }

        // println!("{}: {} : {}", c, counter, mult_pos(pos));

        pos += 1;
    }
    return counter;
}

fn mult_pos(p: u16) -> u32 {
    return u32::pow(16, p as u32);
}

pub fn dec2hex(data: &usize) -> String {
    let mut result = String::new();
    
    // do stuff
    
    return result;
}