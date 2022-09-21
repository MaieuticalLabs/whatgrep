use std::error::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

use clap::{Arg, Command};
use whatlang::Lang;

use whatgrep::match_lines;
use whatgrep::matcher::Matcher;

fn main() -> Result<(), Box<dyn Error>> {
    let m = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about("Search lines for a given language")
        .arg(
            Arg::new("invert_match")
                .short('v')
                .long("invert_match")
                .help("select non-matching lines")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("threshold")
                .short('t')
                .long("threshold")
                .help("from 0.0 to 1.0 confidence threshold")
                .takes_value(true)
                .required(false)
                .default_value("0.6")
                .value_parser(clap::value_parser!(f64)),
        )
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .help("target language")
                .takes_value(true)
                .required(false)
                .default_value("eng"),
        )
        .arg(
            Arg::new("pool")
                .short('p')
                .long("pool")
                .help("command-separated list of possible languages")
                .required(false)
                .takes_value(true)
                .multiple_values(true)
                .value_delimiter(','),
        )
        .arg(
            Arg::new("input")
                .help("input file (\"-\" for stdin)")
                .required(true)
                .takes_value(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .get_matches();

    let input: &PathBuf = m.get_one("input").ok_or("Missing input file")?;
    let str_pool: Vec<String> = m.get_many("pool").unwrap_or_default().cloned().collect();
    let language = m
        .get_one::<String>("language")
        .expect("not found target language argument")
        .to_owned();
    let threshold = m.get_one::<f64>("threshold").unwrap().to_owned();
    let invert_match = m
        .get_one::<bool>("invert_match")
        .expect("not found invert match argument")
        .to_owned();

    let reader: Box<dyn io::Read> = match input.to_str().ok_or("Invalid filename encoding")? {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input)?),
    };

    let mut buffer = BufReader::new(reader);
    let mut content = String::new();
    buffer.read_to_string(&mut content)?;

    let lang = Lang::from_code(language).ok_or("Unable to parse language")?;

    let mut pool: Vec<Lang> = str_pool.iter().filter_map(Lang::from_code).collect();
    pool.push(lang);

    let matcher = Matcher::new(pool, lang, threshold, invert_match);

    let matches: Vec<&str> = match_lines(&content, &matcher);
    for m in &matches {
        println!("{:#}", m);
    }
    Ok(())
}
