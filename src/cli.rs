use std::path::PathBuf;

use clap::{Args, Parser};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[clap(long, short)]
    pub debug: bool,

    #[command(flatten)]
    pub input: InputChoiceGroup,

    #[clap(long, short, name = "output")]
    pub output_file: Option<std::path::PathBuf>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct InputChoiceGroup {
    #[arg(name = "input")]
    pub str: Option<String>,

    #[arg(short('f'), long("file"))]
    pub path: Option<PathBuf>,
}


pub fn parse_cli() -> Cli {
    return Cli::parse();
} 