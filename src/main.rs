use crate::math::offset::*;
use crate::io::{cli::*, file::*};
use crate::util::{*, err::*};

use env_logger::Env;
use log::{error, warn};

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

mod io;
mod math;
mod util;

const COLOR_PRINT_LINE_MIN: usize = 10;

fn main() {
    let args = parse_cli();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);

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

    let mut result: Vec<String> = vec![];

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
        while i < lines.len() {
            
            let a = match conv2dec(&lines[i]) {
                Ok(dec) => dec,
                Err(e) => {
                    if args.fail_fast {
                        error!("[line:{}] Conversion Error: {}", i+1, e);
                        std::process::exit(1);
                    }

                    warn!("[line:{}] Conversion Error: {}", i+1, e);
                    
                    i += 2; continue;
                }
            };

            let b = match conv2dec(&lines[i+1]) {
                Ok(dec) => dec,
                Err(e) => {
                    if args.fail_fast {
                        error!("[line:{}] Conversion Error: {}", i+2, e);
                        std::process::exit(1);
                    }

                    warn!("[line:{}] Conversion Error: {}", i+2, e);

                    i += 2; continue;
                }
            };

            let offset = match conv2base(calc_offset(a,b), target_base) {
                Ok(s) => s,
                Err(e) => {
                    if args.fail_fast {
                        error!("[lines:{},{}] Conversion Error: {}", i+1,i+2, e);
                        std::process::exit(1);
                    }

                    warn!("[lines:{},{}] Conversion Error: {}", i+1,i+2, e);

                    i += 2; continue;
                }
            };

            result.push(offset);
            
            i += 2;
        }
    } else {
        let mut i: usize = 1;
        for line in lines {
            match process_line(&line, target_base) {
                Ok(st) => {
                    result.push(st);
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

    match args.output_file {
        Some(path) => {
            write_lines_to_file(path, result);
        },
        None => {

            // TODO: Better print formatting
            
            let mut i: usize = 0;
            for line in &result {
                if result.len() >= COLOR_PRINT_LINE_MIN {
                    
                    let color: Color;
                    
                    if i % 2 == 0 {
                        color = Color::White;
                    } else {
                        color = Color::Rgb(150, 150, 150);
                    }
                    
                    stdout.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
                    writeln!(&mut stdout, "{}", line).unwrap();
                    stdout.reset().unwrap();

                } else {
                    println!("{}", line);
                }

                i += 1;
            }
        }
    }

}

fn process_line(line: &String, target_base: usize) -> Result<String, ConversionErrors> {

    let decimal_val = match conv2dec(&line) {
        Ok(i) => i,
        Err(e) => return Err(e)
    };

    return conv2base(decimal_val, target_base);
}
