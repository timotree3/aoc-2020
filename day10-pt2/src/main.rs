use std::io::BufRead;

use num::BigUint;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut data: Vec<usize> = handle
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    data.sort_unstable();
    let mut a = BigUint::from(1_u32);
    let mut b = if data[0] == 1 {
        a.clone()
    } else {
        BigUint::from(0_u32)
    };
    let mut c = if data[..2].contains(&2) {
        &a + &b
    } else {
        BigUint::from(0_u32)
    };
    let mut c_joltage = 2;
    for joltage in data {
        if joltage >= 3 {
            match joltage - c_joltage {
                1 => {
                    let new_c = &a + &b + &c;
                    a = b;
                    b = c;
                    c = new_c;
                }
                2 => {
                    let new_c = &b + &c;
                    a = c;
                    b = BigUint::from(0_u32);
                    c = new_c;
                }
                3 => {
                    a = BigUint::from(0_u32);
                    b = BigUint::from(0_u32);
                }
                _ => panic!(),
            }
            c_joltage = joltage;
        }
    }

    println!("answer: {}", c);
}
