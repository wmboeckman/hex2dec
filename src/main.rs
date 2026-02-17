use crate::math::{context::*, conversion::*};
use crate::io::{buff_reader::*, cli::*};
use crate::util::err::*;

use env_logger::Env;
use log::{error, warn};

mod io;
mod math;
mod util;

fn main() {

    let args = parse_cli();

    env_logger::Builder::from_env(
        Env::default()
        .default_filter_or(match args.debug {
            true => "debug",
            false => "error"
        }))
        .format_timestamp(None)
        .format_target(false)
        .init();
    
    let target_base = match args.base {
        Some(i) => i,
        None => 10, // default will be decimal (base-10)
    };
    
    let mut result = String::new();

    // String input XOR Path?
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
                    Ok(l) => l,
                    Err(e) => {
                        error!("File IO Errpr: {}", e);
                        continue;
                    },
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

    // TODO: impliment file writing option
    println!("{}", result);
}


fn process_line(line: &String, target_base: usize) -> Result<String, ConversionErrors>{

    let line_sanitized = util::sanitize_string(line);

    // identify and retrieve base context 
    let possible_context = util::discover_base(&line_sanitized);

    let decimal_val = match possible_context {
        Some(context) => {
            match base2dec(&line_sanitized, context.base, context.charset, &context.prefix) {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        None => match line_sanitized.parse::<usize>() {
            Ok(i) => i,
            Err(_e) => return Err(ConversionErrors::IntConversionError)
        }
    }; 

    let convert_result = match target_base {

        10 => Ok(decimal_val.to_string()),
        
        n =>  {
            for context in CONTEXT_REGISTRY.contexts {
                if context.base == n {
                    return dec2base(&decimal_val, context.base, context.charset, &context.prefix);
                }
            }
            return Err(ConversionErrors::UndefinedBaseContextError);
        }
    };

    return convert_result;
}