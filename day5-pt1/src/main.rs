use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let answer: u64 = input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .rev()
                .enumerate()
                .map(|(i, chr)| if chr == 'B' || chr == 'R' { 1 } else { 0 } << i)
                .sum()
        })
        .max()
        .unwrap();
    println!("answer: {}", answer);
}
