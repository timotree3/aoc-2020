use std::{collections::HashSet, convert::TryFrom, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let ops: Vec<Op> = input.lines().map(|line| parse_op(&line.unwrap())).collect();
    let mut visited: HashSet<i64> = HashSet::new();
    let mut ip: i64 = 0;
    let mut acc = 0;
    loop {
        let op = &ops[usize::try_from(ip).unwrap()];
        match op {
            Op::Acc(amount) => {
                acc += amount;
                ip += 1;
            }
            Op::Jmp(offset) => ip += offset,
            Op::Nop => ip += 1,
        }
        if visited.contains(&ip) {
            break;
        }
        visited.insert(ip);
    }
    println!("answer: {}", acc);
}

fn parse_op(text: &str) -> Op {
    let instruction = &text[..3];
    let sign = &text[4..5];
    let operand_text = if sign == "-" { &text[4..] } else { &text[5..] };
    match instruction {
        "acc" => Op::Acc(operand_text.parse().unwrap()),
        "jmp" => Op::Jmp(operand_text.parse().unwrap()),
        "nop" => Op::Nop,
        _ => panic!(),
    }
}

enum Op {
    Acc(i64),
    Jmp(i64),
    Nop,
}
