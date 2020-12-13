use std::io::Read;

use snafu::{ensure, OptionExt, ResultExt, Snafu};

fn main() -> Result<(), Error> {
    let grid = read_grid()?;
    let mut x = 0;
    let mut line_start = 0;
    let mut trees = 0;
    while line_start != grid.data.len() {
        let index = line_start + x;
        if grid.data[index] == b'#' {
            trees += 1;
        }
        x = (x + 3) % grid.width;
        line_start += grid.width + 1;
    }
    println!("answer: {}", trees);
    Ok(())
}

struct Grid {
    data: Vec<u8>,
    width: usize,
}

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Input length must be divisible by the length of the first line"))]
    UnevenLines,
    #[snafu(display("Could not read data from stdin: {}", source))]
    ReadStdin { source: std::io::Error },
    #[snafu(display("Input must contain a newline"))]
    ZeroLinebreaks,
}

fn read_grid() -> Result<Grid, Error> {
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).context(ReadStdin)?;
    let line_length = data
        .iter()
        .position(|&b| b == b'\n')
        .context(ZeroLinebreaks)?;
    ensure!(data.len() % (line_length + 1) == 0, UnevenLines);
    Ok(Grid {
        data,
        width: line_length,
    })
}
