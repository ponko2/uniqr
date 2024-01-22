use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Input file", default_value = "-")]
    in_file: String,

    #[arg(help = "Output file")]
    out_file: Option<String>,

    #[arg(short, long, help = "Show counts")]
    count: bool,
}

pub fn get_args() -> Result<Args> {
    Ok(Args::parse())
}

pub fn run(args: Args) -> Result<()> {
    dbg!(args);
    Ok(())
}
