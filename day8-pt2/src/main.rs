use std::{collections::HashSet, convert::TryFrom, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let input = stdin.lock();
    let mut ops: Vec<Op> = input.lines().map(|line| parse_op(&line.unwrap())).collect();
    println!("answer: {}", answer(&mut ops));
}

fn answer(ops: &mut [Op]) -> i64 {
    for i in 0..ops.len() {
        match &mut ops[i].variant {
            op_type @ OpType::Jmp => *op_type = OpType::Nop,
            op_type @ OpType::Nop => *op_type = OpType::Jmp,
            OpType::Acc => continue,
        }
        let (terminated, acc) = simulate(&ops);
        if terminated {
            return acc;
        }
        match &mut ops[i].variant {
            op_type @ OpType::Jmp => *op_type = OpType::Nop,
            op_type @ OpType::Nop => *op_type = OpType::Jmp,
            OpType::Acc => unreachable!(),
        }
    }
    panic!()
}

fn simulate(ops: &[Op]) -> (bool, i64) {
    let mut visited: HashSet<i64> = HashSet::new();
    let mut ip: i64 = 0;
    let mut acc = 0;
    let terminated = loop {
        let op = &ops[usize::try_from(ip).unwrap()];
        match op.variant {
            OpType::Acc => {
                acc += op.arg;
                ip += 1;
            }
            OpType::Jmp => ip += op.arg,
            OpType::Nop => ip += 1,
        }
        if usize::try_from(ip).unwrap() >= ops.len() {
            break true;
        }
        if visited.contains(&ip) {
            break false;
        }
        visited.insert(ip);
    };
    (terminated, acc)
}

fn parse_op(text: &str) -> Op {
    let instruction = &text[..3];
    let sign = &text[4..5];
    let operand_text = if sign == "-" { &text[4..] } else { &text[5..] };
    let operand = operand_text.parse().unwrap();
    Op {
        variant: match instruction {
            "acc" => OpType::Acc,
            "jmp" => OpType::Jmp,
            "nop" => OpType::Nop,
            _ => panic!(),
        },
        arg: operand,
    }
}

struct Op {
    variant: OpType,
    arg: i64,
}

enum OpType {
    Acc,
    Jmp,
    Nop,
}
