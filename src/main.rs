mod err;
mod cli;
mod math;
mod tests;

fn main() {
    let args = cli::parse_cli();
    
    if args.debug {
        //set a global debug flag?
    }

    // process input branch here
    match &args.input.str {
        Some(p) => println!("{}", p),
        None => println!("No String!"),
    }

    match &args.input.path {
        Some(p) => println!("{}",p.to_str().unwrap()),
        None => println!("No Path!"),
    }
    
    let result = math::hex2dec(&args.input.str.unwrap());

    // for now, just print
    match result {
        Ok(n) => println!("{}", n),
        Err(e) => println!("ERR: {}", e)
    }
}  