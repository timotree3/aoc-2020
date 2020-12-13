#![feature(str_split_once)]
use std::{collections::HashSet, io::Read};

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut found_new;
    let mut interesting = HashSet::with_capacity(1);
    interesting.insert("shiny gold");
    loop {
        found_new = false;
        for rule in data.lines() {
            let (name, rest) = rule.split_once(" bags").unwrap();
            if interesting.contains(&name) {
                continue;
            }
            if interesting.iter().any(|bag| rest.contains(bag)) {
                interesting.insert(name);
                found_new = true;
            }
        }
        if !found_new {
            break;
        }
    }
    println!("answer: {}", interesting.len() - 1);
}
