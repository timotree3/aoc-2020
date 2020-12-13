#![feature(str_split_once)]
use std::{collections::HashMap, io::Read};

use regex::Regex;

fn main() {
    let rule_pattern = Regex::new(r"^([a-z ]+) bags? contain (.+)\.$").unwrap();
    let separator_pattern = Regex::new(" bags?(, )?").unwrap();
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let rules = data
        .lines()
        .map(|rule| {
            let captures = rule_pattern.captures(rule).unwrap();
            let subject = &captures[1];
            let amounts_text = &captures[2];
            let pairs: Vec<(u64, String)> = if amounts_text == "no other bags" {
                Vec::new()
            } else {
                let amounts = separator_pattern.split(amounts_text);
                amounts
                    .filter(|text| !text.is_empty())
                    .map(|text| {
                        let (quantity, bag_type) = text.split_once(" ").unwrap();
                        (quantity.parse().unwrap(), bag_type.to_string())
                    })
                    .collect()
            };

            (subject.to_string(), pairs)
        })
        .collect();
    println!("answer: {}", calc_children(&rules, "shiny gold") - 1);
}

fn calc_children(rules: &HashMap<String, Vec<(u64, String)>>, name: &str) -> u64 {
    rules
        .get(name)
        .unwrap()
        .iter()
        .map(|(quantity, bag_type)| calc_children(rules, bag_type) * quantity)
        .sum::<u64>()
        + 1
}
