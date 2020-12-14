#![feature(str_split_once)]
use std::{collections::HashMap, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut mask_on: usize = 0;
    let mut mask_off = usize::MAX;
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in handle.lines() {
        let line = line.unwrap();
        let (place, value) = line.split_once(" = ").unwrap();
        if place == "mask" {
            mask_on = 0;
            mask_off = 0;
            for (i, chr) in value.chars().rev().enumerate() {
                match chr {
                    'X' => {}
                    '1' => mask_on |= 1 << i,
                    '0' => mask_off |= 1 << i,
                    _ => panic!(),
                }
            }
            mask_off = !mask_off;
            println!(
                "value: {}, mask_on: {:b}, mask_off: {:b}",
                value, mask_on, mask_off
            )
        } else {
            let address = &place[4..(place.len() - 1)];
            let address = address.parse().unwrap();
            let mut value = value.parse().unwrap();
            value |= mask_on;
            value &= mask_off;
            memory.insert(address, value);
        }
    }
    println!("answer: {}", memory.values().sum::<usize>());
}
