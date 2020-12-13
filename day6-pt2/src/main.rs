use std::{collections::HashSet, io::Read};

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let sum: usize = data
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(None, |state, person| {
                    let set = person.chars().collect::<HashSet<_>>();
                    match state {
                        Some(set2) => Some(set.intersection(&set2).cloned().collect()),
                        None => Some(set),
                    }
                })
                .map(|set| set.len())
                .unwrap()
        })
        .sum();
    println!("answer: {}", sum);
}
