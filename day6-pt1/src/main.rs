use std::{collections::HashSet, io::Read};

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let sum: usize = data
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum();
    println!("answer: {}", sum);
}
