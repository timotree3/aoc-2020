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
    let ints_result: Result<Vec<u32>, Error> = input
        .lines()
        .map(|line_result| {
            line_result
                .context(ReadLine)
                .and_then(|line| line.parse().context(ParseLine { line }))
        })
        .collect();
    let mut slice: &[u32] = &ints_result?;
    loop {
        let (last, head) = slice.split_last().ok_or(Error::NotFound)?;
        if let Some(answer) = pass(head, *last) {
            return Ok(answer);
        }
        slice = head
    }
}

fn pass(input: &[u32], candidate: u32) -> Option<u32> {
    let mut seen = HashSet::new();
    let candidate_complement = 2020 - candidate;
    for int in input {
        let complement_opt = candidate_complement.checked_sub(*int);
        if let Some(complement) = complement_opt {
            if seen.contains(&complement) {
                return Some(complement * int * candidate);
            }
            seen.insert(int);
        }
    }
    None
}
