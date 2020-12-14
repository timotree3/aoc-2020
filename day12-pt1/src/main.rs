use std::io::BufRead;

fn main() {
    let mut rot = 0;
    let mut up = 0;
    let mut right = 0;
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        let (letter, number) = line.split_at(1);
        let number: isize = number.parse().unwrap();
        match letter {
            "N" => up += number,
            "S" => up -= number,
            "E" => right += number,
            "W" => right -= number,
            "L" => rot = (rot + number) % 360,
            "R" => rot = (rot + 360 - number) % 360,
            "F" => {
                up += sin(rot) * number;
                right += cos(rot) * number
            }
            _ => panic!(),
        }
    }
    println!("answer: {}", up.abs() + right.abs());
}

fn sin(rot: isize) -> isize {
    match rot {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => panic!("unexpected rotation: {}", rot),
    }
}

fn cos(rot: isize) -> isize {
    match rot {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("unexpected rotation: {}", rot),
    }
}
