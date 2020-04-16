extern crate structopt;
extern crate whatlang;

mod matcher;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use structopt::StructOpt;
use whatlang::Lang;

use matcher::Matcher;

#[derive(Debug, StructOpt)]
#[structopt(name = "Whatgrep", about = "Search lines for a given language")]
struct Opt {
    /// Select non-matching lines
    #[structopt(short = "v", long = "invert_match")]
    invert_match: bool,

    /// from 0.0 to 1.0 confidence threshold
    #[structopt(short = "t", long = "threshold", default_value = "0.6")]
    threshold: f64,

    /// Target language
    #[structopt(short = "l", long = "language", default_value = "eng")]
    language: String,

    /// Whitelist of possible languages (recommended for short texts)
    #[structopt(short = "p", long = "pool")]
    pool: Vec<String>,

    /// Input file ("-" for stdin)
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();
    let reader: Box<dyn io::Read> = match opt.input.to_str().ok_or("Invalid filename encoding")? {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(opt.input)?),
    };

    let buffer = BufReader::new(reader);

    let lang = Lang::from_code(opt.language).ok_or("Unable to parse language")?;

    let mut pool: Vec<Lang> = opt.pool.iter().filter_map(Lang::from_code).collect();
    pool.push(lang);

    let matcher = Matcher::new(pool, lang, opt.threshold, opt.invert_match);

    for line_res in buffer.lines() {
        let line = line_res?;
        if matcher.is_lang(&line) {
            println!("{:#}", &line);
        }
    }
    Ok(())
}
