extern crate clap;
extern crate rayon;
extern crate whatlang;

pub mod matcher;

use matcher::Matcher;
use rayon::prelude::*;

pub fn match_lines<'a>(lines: &'a str, matcher: &Matcher) -> Vec<&'a str> {
    lines
        .par_lines()
        .filter(|line| matcher.is_lang(line))
        .collect()
}
