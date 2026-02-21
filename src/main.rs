use crate::math::offset::*;
use crate::io::{cli::*, lines_from_file};
use crate::util::{*, err::*};

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
            false => "warn"
        }))
        .format_timestamp(None)
        .format_target(false)
        .init();
    
    let target_base = match args.base {
        Some(i) => i,
        None => 10, // default will be decimal (base-10)
    };
    
    let mut result = String::new();

    let lines = match args.input {
        
        InputChoiceGroup {
        str: Some(inputs),
        path: None
        } => inputs,

        InputChoiceGroup {
        str: None,
        path: Some(p)
        } => lines_from_file(p),

        _ => panic!("HOW DID YOU GET HERE"), // clap should make this impossible to reach
    };

    // offset feature requires even number of inputs
    if args.offset {
        if lines.len() % 2 != 0 {
            error!("Offset requires an even number of inputs!");
            std::process::exit(1);
        }

        // iter. groups of 2
        let mut i: usize = 1;
        while i < lines.len() / 2 {
            
            let a = match conv2dec(&lines[i]) {
                Ok(i) => i,
                Err(e) => {
                    if args.fail_fast {
                        error!("[line:{}] Conversion Error: {}", i, e);
                        std::process::exit(1);
                    }

                    warn!("[line:{}] Conversion Error: {}", i, e);
                    
                    i += 2; continue;
                }
            };

            let b = match conv2dec(&lines[i+1]) {
                Ok(i) => i,
                Err(e) => {
                    if args.fail_fast {
                        error!("[line:{}] Conversion Error: {}", i+1, e);
                        std::process::exit(1);
                    }

                    warn!("[line:{}] Conversion Error: {}", i+1, e);

                    i += 2; continue;
                }
            };

            result.push_str(&conv2base(calc_offset(a,b), target_base).unwrap());
            result.push('\n');
            
            i += 2;
        }
    } else {
        let mut i: usize = 1;
        for line in lines {
            match process_line(&line, target_base) {
                Ok(st) => {
                    result.push_str(&st);
                    result.push('\n');
                },
                Err(e) => {
                    if args.fail_fast {
                        error!("[line:{}] Conversion Error: {}", i, e);
                        std::process::exit(1);
                    }
    
                    warn!("[line:{}] Conversion Error: {}", i, e);
                }
            }
            i += 1;
        }
    }

    // TODO: impliment file writing option
    print!("{}", result);
}

fn process_line(line: &String, target_base: usize) -> Result<String, ConversionErrors> {
    
    debug!("\"{}\" -> \"{}\"", line.trim_end(), &line);

    let decimal_val = match conv2dec(&line) {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    return conv2base(decimal_val, target_base);
}
