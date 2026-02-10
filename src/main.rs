use std::env;

mod cli;
mod math;
mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    cli::cli(args).unwrap();
}

