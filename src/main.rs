use crate::{cli::InputChoiceGroup, err::ConversionErrors, io::BufReader, math::{base2dec, dec2base}};
use log::{error, warn};

mod cli;
mod err;
mod io;
mod math;
mod tests;
mod util;

struct BaseContext<'a> {
    base: usize,
    charset: &'a [char],
    prefix: [char; 2]
} 

const CONTEXT_B16: BaseContext = BaseContext {
    base: 16,
    charset: &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'],
    prefix: ['0', 'x']
};

const CONTEXT_B8: BaseContext = BaseContext {
    base: 8,
    charset: &['0', '1', '2', '3', '4', '5', '6', '7'],
    prefix: ['0', 'o']
};

fn main() {
    env_logger::init();

    let args = cli::parse_cli();

    let mut result = String::new();

    if args.debug {
        println!("Debug flag enabled!");
    }

    let target_base = match args.base {
        Some(i) => i,
        None => 10, // default will be hexadecimal
    };

    // TODO: cli args to check for known bases

    match args.input {
        InputChoiceGroup {
            str: Some(s),
            path: None,
        } => {
            result = match process_line(&s, target_base) {
                Ok(st) => st,
                Err(e) => {
                    error!("Conversion Error: {}", e);
                    std::process::exit(1);
                }
            };
        }
        InputChoiceGroup {
            str: None,
            path: Some(p),
        } => {
            // attempt to open file from provided path, return a buffer reader
            let br = match BufReader::open(p) {
                Ok(r) => r,
                Err(e) => {
                    error!("File IO Error: {}", e);
                    std::process::exit(1);
                }
            };

            for line in br {
                let clean_line = match line {
                    Ok(l) => util::sanitize_string(&l),
                    Err(e) => panic!("{}", e), //bad, I know
                };

                match process_line(&clean_line, target_base) {
                    Ok(s) => {
                        // save the output to a string buffer to send as output after debug messages
                        result.push_str(&s);
                        result.push('\n');
                    }
                    Err(e) => {
                        if args.debug {
                            warn!("Conversion Error: {}. Continuing", e);
                        }

                        // TODO: add flag to early exit batch processing if error occurs!
                    }
                }
            }
        }
        _ => panic!("HOW DID YOU GET HERE"), // clap should make this impossible to reach
    };

    // for now, just print
    println!("{}", result);
}


fn process_line(line: &String, target_base: usize) -> Result<String, ConversionErrors>{

    let line_sanitized = util::sanitize_string(line);

    // identify and retrieve base context 
    let possible_context = util::discover_base(&line_sanitized);

    let decimal_val = match possible_context {
        Some((charset, prefix)) => {
            match base2dec(&line_sanitized, charset.len(), charset, prefix) {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        None => match line_sanitized.parse::<usize>() {
            Ok(i) => i,
            Err(_e) => return Err(ConversionErrors::InvalidCharError)
        }
    }; 

    let convert_result = match target_base {
        16 => dec2base(&decimal_val, CONTEXT_B16.base, &CONTEXT_B16.charset, &CONTEXT_B16.prefix),
        8 => dec2base(&decimal_val, CONTEXT_B8.base, &CONTEXT_B8.charset, &CONTEXT_B8.prefix),
        10 => Ok(decimal_val.to_string()),
        _ => return Err(ConversionErrors::InvalidBasePrefixError)
    };

    return convert_result;
}