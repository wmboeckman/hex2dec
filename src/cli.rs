use crate::math;

pub fn cli(args: Vec<String>) -> Result<(), &'static str> {
    
    if args.len() < 2 {
        // display help page
        return Ok(());
    }

    let arg1: &String = &args[1];

    if !is_valid(&arg1) {
        return Err("Invalid Argument(s)!");
    }

    let result = math::hex2dec(&args[1]);

    println!("{}", result); // for now just print

    return Ok(());
}


pub fn is_valid(data: &String) -> bool {
    
    let valid_chars: String = String::from("0123456789abcdef"); // all valid hexadecimal chars

    return data.chars().all(
        |c: char| match valid_chars.find(c) {
            Some(_index) => true,
            None => false,
        }
    );
}