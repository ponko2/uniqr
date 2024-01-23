use anyhow::{anyhow, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

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
    let mut file = open(&args.in_file).map_err(|err| anyhow!("{}: {err}", args.in_file))?;
    let mut buf = String::new();
    let mut previous = String::new();
    let mut count = 0;
    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(filename) => Box::new(File::create(filename)?),
        _ => Box::new(io::stdout()),
    };
    let mut print = |count: u64, text: &str| -> Result<()> {
        if count > 0 {
            if args.count {
                write!(out_file, "{count:>4} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        }
        Ok(())
    };
    while file.read_line(&mut buf)? > 0 {
        if buf.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = buf.clone();
            count = 0;
        }
        count += 1;
        buf.clear();
    }
    print(count, &previous)?;
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    Ok(match filename {
        "-" => Box::new(BufReader::new(io::stdin())),
        _ => Box::new(BufReader::new(File::open(filename)?)),
    })
}
