use std::{convert::TryInto, io::BufRead};

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
        .filter(|r| r.as_ref().map_err(|e| eprintln!("{}", e)) == Ok(&true))
        .count()
}

fn validate_line(line_res: Result<String, std::io::Error>) -> Result<bool, Error> {
    let line = line_res.context(ReadLine)?;

    // 1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    static PATTERN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap());

    let captures = match PATTERN.captures(&line) {
        Some(c) => c,
        None => return Err(Error::ParseLine { line }),
    };

    let pos1_str = &captures[1];
    let pos1: usize = pos1_str.parse().with_context(|| IntegerDecode {
        input: pos1_str.to_owned(),
    })?;

    let pos2_str = &captures[2];
    let pos2: usize = pos2_str.parse().with_context(|| IntegerDecode {
        input: pos2_str.to_owned(),
    })?;
    let &[req_letter]: &[u8; 1] = captures[3]
        .as_bytes()
        .try_into()
        .expect("[a-z] pattern should match exactly one byte");
    let password = captures[4].as_bytes();
    Ok((password[pos1 - 1] == req_letter) ^ (password[pos2 - 1] == req_letter))
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read line from stdin: {}", source))]
    ReadLine { source: std::io::Error },
    #[snafu(display("Could not parse line using regex {}", line))]
    ParseLine { line: String },
    #[snafu(display("Could not decode text into integer {}: {}", input, source))]
    IntegerDecode {
        input: String,
        source: std::num::ParseIntError,
    },
}
