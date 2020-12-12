use snafu::{ResultExt, Snafu};
use std::collections::HashSet;
use std::io::BufRead;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Could not read next line from stdin: {}", source))]
    ReadLine { source: std::io::Error },
    #[snafu(display("Could not parse line as integer {}: {}", line, source))]
    ParseLine {
        line: String,
        source: std::num::ParseIntError,
    },
    #[snafu(display("Could not find a valid pair of integers"))]
    NotFound,
}

fn main() {
    let stdin = std::io::stdin();
    let mut input = stdin.lock();
    println!("{}", run(&mut input).unwrap());
}

fn run<R: BufRead>(input: R) -> Result<u32, Error> {
    let mut seen = HashSet::new();
    for line in input.lines() {
        let line = line.context(ReadLine)?;
        let int: u32 = line.parse().context(ParseLine { line })?;
        let complement = 2020 - int;
        if seen.contains(&complement) {
            return Ok(int * complement);
        }
        seen.insert(int);
    }
    Err(Error::NotFound)
}
