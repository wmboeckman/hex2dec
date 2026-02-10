use crate::math;

pub fn cli(args: Vec<String>) -> Result<(), &'static str> {
    
    if args.len() < 2 {
        // display help page
        return Ok(());
    }

    let arg1: &String = &args[1];

    let result = math::hex2dec(arg1);

    // for now, just print
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("ERR: {}", e)
    }

    return Ok(());
}
