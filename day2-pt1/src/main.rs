use std::{io::BufRead, ops::RangeInclusive, str::FromStr};

use once_cell::sync::Lazy;
use regex::Regex;
use snafu::{ResultExt, Snafu};

fn main() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    println!("answer: {}", run(&mut input));
}

fn run<R: BufRead>(input: R) -> usize {
    input
        .lines()
        .map(validate_line)
        .filter(|res| {
            *res.as_ref().unwrap_or_else(|err| {
                eprintln!("{}", err);
                &false
            })
        })
        .count()
}

fn validate_line(line_res: Result<String, std::io::Error>) -> Result<bool, Error> {
    let line = line_res.context(ReadLine)?;
    let record: Record = line.parse().with_context(|| ParseLine { line })?;
    let occurrences = record
        .password
        .chars()
        .filter(|&c| c == record.req_letter)
        .count();
    Ok(record.range.contains(&occurrences))
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read line from stdin: {}", source))]
    ReadLine { source: std::io::Error },
    #[snafu(display("Could not parse line {}: {}", line, source))]
    ParseLine { line: String, source: FromStrError },
}

#[derive(Debug, Snafu)]
enum FromStrError {
    #[snafu(display("Could not parse input using regex"))]
    Parse,
    #[snafu(display("Could not decode text into integer {}: {}", input, source))]
    IntegerDecode {
        input: String,
        source: std::num::ParseIntError,
    },
}

struct Record {
    range: RangeInclusive<usize>,
    req_letter: char,
    password: String,
}

impl FromStr for Record {
    type Err = FromStrError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1-3 a: abcde
        // 1-3 b: cdefg
        // 2-9 c: ccccccccc
        static PATTERN: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap());
        let captures = PATTERN.captures(s).ok_or(FromStrError::Parse)?;

        let min_str = &captures[1];
        let min = min_str.parse().with_context(|| IntegerDecode {
            input: min_str.to_owned(),
        })?;

        let max_str = &captures[2];
        let max = max_str.parse().with_context(|| IntegerDecode {
            input: max_str.to_owned(),
        })?;
        let req_letter = captures[3]
            .chars()
            .next()
            .expect("[a-z] pattern should match at least one char");
        let password = captures[4].to_string();
        Ok(Self {
            range: min..=max,
            req_letter,
            password,
        })
    }
}
