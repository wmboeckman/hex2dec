use crate::math::{context::*, conversion::*, offset::*};
use crate::io::{cli::*, lines_from_file};
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
        let mut i: usize = 0;
        while i <= lines.len() / 2 {
            
            let a = match conv2dec(&util::sanitize_string(&lines[i])) {
                Ok(i) => i,
                Err(e) => {
                    if args.fail_fast {
                        error!("[stdin:{}] Conversion Error: {}", i, e);
                        std::process::exit(1);
                    }

                    warn!("[stdin:{}] Conversion Error: {}", i, e);
                    
                    i += 2; continue;
                }
            };

            let b = match conv2dec(&util::sanitize_string(&lines[i+1])) {
                Ok(i) => i,
                Err(e) => {
                    if args.fail_fast {
                        error!("[stdin:{}] Conversion Error: {}", i+1, e);
                        std::process::exit(1);
                    }

                    warn!("[stdin:{}] Conversion Error: {}", i+1, e);

                    i += 2; continue;
                }
            };

            result.push_str(&conv2base(calc_offset(a,b), target_base).unwrap());
            result.push('\n');
            
            i += 2;
        }
    } else {
        let mut i: usize = 0;
        for s in lines {
            match process_line(&s, target_base) {
                Ok(st) => {
                    result.push_str(&st);
                    result.push('\n');
                },
                Err(e) => {
                    if args.fail_fast {
                        error!("[stdin:{}] Conversion Error: {}", i, e);
                        std::process::exit(1);
                    }
    
                    warn!("[stdin:{}] Conversion Error: {}", i, e);
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

// TODO: Move below functions to 'util' module

fn conv2dec(line: &String) -> Result<usize, ConversionErrors> {
    let line_sanitized = util::sanitize_string(line);
    
    let possible_context = util::discover_base(&line_sanitized);

    let decimal_val = match possible_context {
        Some(context) => {
            match base2dec(&line_sanitized, context.base, context.charset, &context.prefix) {
                Ok(i) => i,
                Err(e) => return Err(e)
            }
        },
        None => match line_sanitized.parse::<usize>() { // check for base-10 decimal
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