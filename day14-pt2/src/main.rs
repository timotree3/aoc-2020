#![feature(str_split_once)]
use std::{collections::HashMap, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut mask = String::new();
    let mut memory: HashMap<usize, usize> = HashMap::new();
    for line in handle.lines() {
        let line = line.unwrap();
        let (place, value) = line.split_once(" = ").unwrap();
        if place == "mask" {
            mask = value.to_owned();
        } else {
            let address = &place[4..(place.len() - 1)];
            let address = address.parse().unwrap();
            rec(
                &mut memory,
                address,
                value.parse().unwrap(),
                mask.as_bytes(),
                0,
            )
        }
    }
    println!("answer: {}", memory.values().sum::<usize>());
}

fn rec(
    memory: &mut HashMap<usize, usize>,
    address: usize,
    value: usize,
    mask: &[u8],
    index: usize,
) {
    if index >= mask.len() {
        memory.insert(address, value);
        return;
    }
    match mask[mask.len() - 1 - index] {
        b'0' => rec(memory, address, value, mask, index + 1),
        b'1' => rec(memory, address | 1 << index, value, mask, index + 1),
        b'X' => {
            rec(memory, address | 1 << index, value, mask, index + 1);
            rec(memory, address & !(1 << index), value, mask, index + 1);
        }
        _ => panic!(),
    }
}
