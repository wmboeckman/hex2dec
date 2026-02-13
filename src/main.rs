use std::{fs::File, io::Read};

mod err;
mod cli;
mod math;
mod tests;

fn main() {
    let args = cli::parse_cli();
    
    let mut result = String::new();

    if false && args.debug {
        println!("Debug flag enabled!");
    }
    
    // process input branch here
    match &args.input.str {
        Some(p) => {
            match math::hex2dec(p, args.debug) {
                Ok(o) => result = o,
                Err(e) => println!("ERR: {}", e),
            };
            // match math::dec2hex(p, args.debug) {
            //     Ok(o) => result = o,
            //     Err(e) => println!("ERR: {}", e),
            // };
        },
        None => (),
    }

    match &args.input.path {
        Some(p) => {
            
            let mut buffer = String::new();
            
            let mut answers = String::new();

            let mut fs = match File::open(p) {
                Ok(a) => a,
                Err(e) => panic!("file read failure: {}", e)
            };

            fs.read_to_string(&mut buffer).unwrap();

            for line in buffer.split('\n').into_iter() {
                let a = math::hex2dec(&line.to_owned(), args.debug);
                
                match a {
                    Ok(d) => answers.push_str(&d),
                    Err(e) => () // could be fine maybe :)
                }
            }
            
            result = answers;
        },
        None => (),
    }

    // for now, just print
    println!("{}", result);
}  