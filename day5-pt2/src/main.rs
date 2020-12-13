use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let mut ids: Vec<u16> = input
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .rev()
                .enumerate()
                .map(|(i, chr)| if chr == 'B' || chr == 'R' { 1 } else { 0 } << i)
                .sum()
        })
        .collect();
    ids.sort_unstable();
    let pair = ids
        .windows(2)
        .find(|window| window[0] + 1 != window[1])
        .unwrap();
    assert!(pair[0] + 2 == pair[1]);
    println!("answer: {}", pair[0] + 1);
}
