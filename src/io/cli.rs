use std::path::PathBuf;

use clap::{Args, Parser};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(long, short)]
    pub debug: bool,

    #[clap(long("ffast"), short('f'))]
    pub fail_fast: bool,

    #[clap(long, short)]
    pub offset: bool,

    #[clap(long, short)]
    pub base: Option<usize>,

    #[command(flatten)]
    pub input: InputChoiceGroup,

    #[clap(long("out"), short('O'), name = "PATH")]
    pub output_file: Option<std::path::PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct InputChoiceGroup {
    #[arg(name = "input")]
    pub str: Option<Vec<String>>,

    #[arg(short('I'), long("in"))]
    pub path: Option<PathBuf>,
}


pub fn parse_cli() -> Cli {
    return Cli::parse();
}