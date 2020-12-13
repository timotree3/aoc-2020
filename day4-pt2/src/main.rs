#![feature(str_split_once)]
use once_cell::sync::Lazy;
use regex::Regex;
use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let valid_passports = data
        .split("\n\n")
        .filter(|&passport| {
            dbg!(passport)
                .split_ascii_whitespace()
                .all(|text| dbg!(validate_kv(dbg!(text))))
                && dbg!(
                    passport
                        .split_ascii_whitespace()
                        .filter(|&field| !field.starts_with("cid"))
                        .count()
                        == 7
                )
        })
        .count();
    println!("answer: {}", valid_passports);
}

fn validate_kv(text: &str) -> bool {
    let (key, value) = if let Some(tuple) = text.split_once(":") {
        tuple
    } else {
        return false;
    };
    match key {
        "byr" => value
            .parse()
            .map(|y: i32| (1920..=2002).contains(&y))
            .unwrap_or(false),
        "iyr" => value
            .parse()
            .map(|y: i32| (2010..=2020).contains(&y))
            .unwrap_or(false),
        "eyr" => value
            .parse()
            .map(|y: i32| (2020..=2030).contains(&y))
            .unwrap_or(false),
        "hgt" => {
            static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d+)(cm|in)$").unwrap());
            RE.captures(value)
                .map(|captures| {
                    let amount: u32 = captures.get(1).unwrap().as_str().parse().unwrap();
                    let units = captures.get(2).unwrap();
                    match units.as_str() {
                        "cm" => (150..=193).contains(&amount),
                        "in" => (59..=76).contains(&amount),
                        _ => false,
                    }
                })
                .unwrap_or(false)
        }
        "hcl" => {
            static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^#[0-9a-f]{6}$").unwrap());
            RE.is_match(value)
        }
        "ecl" => {
            static RE: Lazy<Regex> =
                Lazy::new(|| Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap());
            RE.is_match(value)
        }
        "pid" => {
            static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{9}$").unwrap());
            RE.is_match(value)
        }
        "cid" => true,
        _ => false,
    }
}
