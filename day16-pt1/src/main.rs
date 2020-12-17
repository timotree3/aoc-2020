#![feature(str_split_once)]
use std::{io::Read, ops::RangeInclusive};

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut sections = data.split("\n\n");
    let fields = sections.next().unwrap();
    let ranges: Vec<RangeInclusive<usize>> = fields
        .lines()
        .map(|line| {
            let (_name, ranges) = line.split_once(": ").unwrap();
            ranges.split(" or ").map(|range| {
                let (start, end) = range.split_once("-").unwrap();
                start.parse().unwrap()..=end.parse().unwrap()
            })
        })
        .flatten()
        .collect();
    let _my_ticket = sections.next().unwrap();
    let tickets = sections.next().unwrap().lines().skip(1);
    let answer: usize = tickets
        .flat_map(|ticket| {
            ticket
                .split(',')
                .map(|num| num.parse().unwrap())
                .filter(|num| !ranges.iter().any(|range| range.contains(&num)))
        })
        .sum();
    println!("answer: {}", answer);
}
