use crate::math::{context::*, conversion::*, offset::*};
use crate::io::{buff_reader::*, cli::*};
use crate::util::err::*;

use env_logger::Env;
use log::{debug, error, warn};

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

    // TODO: break appart each branch into separate functions
    // String input XOR Path?
    match args.input {
        InputChoiceGroup {
            str: Some(inputs),
            path: None
        } => {
            if args.offset {
                if inputs.len() % 2 != 0 {
                    error!("Offset requires an even number of inputs!");
                    std::process::exit(1);
                }

                let mut i = 0;
                // iter. groups of 2
                while i <= inputs.len() / 2 {
                    
                    let a = match conv2dec(&util::sanitize_string(&inputs[i])) {
                        Ok(i) => i,
                        Err(e) => {
                            warn!("Conversion Error: {}. continuing", e);
                            continue;
                        }
                    };

                    let b = match conv2dec(&util::sanitize_string(&inputs[i+1])) {
                        Ok(i) => i,
                        Err(e) => {
                            warn!("Conversion Error: {}. continuing", e);
                            continue;
                        }
                    };

                    result.push_str(&conv2base(calc_offset(a,b), target_base).unwrap());
                    result.push('\n');
                    
                    i += 2;
                }

            } else {
                for s in inputs {
                    match process_line(&s, target_base) {
                        Ok(st) => {
                            result.push_str(&st);
                            result.push('\n');
                        },
                        Err(e) => {
                            warn!("Conversion Error: {}. Continuing", e);
                        }
                    }
                }
            }

        }
        InputChoiceGroup {
            str: None,
            path: Some(p)
        } => {
            // attempt to open file from provided path, return a buffer reader
            let br = match BufReader::open(p) {
                Ok(r) => r,
                Err(e) => {
                    error!("File IO Error: {}", e);
                    std::process::exit(1);
                }
            };

            // TODO: split lines on whitespace chars 

            for line in br {
                let clean_line = match line {
                    Ok(l) => l,
                    Err(e) => {
                        error!("File IO Error: {}", e);
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
                        warn!("Conversion Error: {}. Continuing", e);
                        
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

    // TODO: skip empty strings
    
    debug!("\"{}\" -> \"{}\"", line.trim_end(), line_sanitized);

    let decimal_val = match conv2dec(&line_sanitized) {
        Ok(i) => i,
        Err(_e) => return Err(ConversionErrors::IntConversionError)
    };

    return conv2base(decimal_val, target_base);
}

fn conv2dec(line: &String) -> Result<usize, ConversionErrors> {
    let possible_context = util::discover_base(&line);

    let decimal_val = match possible_context {
        Some(context) => {
            match base2dec(&line, context.base, context.charset, &context.prefix) {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        None => match line.parse::<usize>() { // check for base-10 decimal
            Ok(i) => i,
            Err(_e) => return Err(ConversionErrors::IntConversionError)
        }
    };

    return Ok(decimal_val);
}

fn conv2base(input: usize, target_base: usize) -> Result<String, ConversionErrors> {
    let convert_result = match target_base {

        10 => Ok(input.to_string()),
        
        n =>  {
            for context in CONTEXT_REGISTRY.contexts {
                if context.base == n {
                    return dec2base(&input, context.base, context.charset, &context.prefix);
                }
            }
            return Err(ConversionErrors::UndefinedBaseContextError);
        }
    };

    return convert_result;
}