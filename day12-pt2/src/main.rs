use std::io::BufRead;

fn main() {
    let mut wp_up = 1;
    let mut wp_right = 10;
    let mut up = 0;
    let mut right = 0;
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let line = line.unwrap();
        let (letter, number) = line.split_at(1);
        let number: isize = number.parse().unwrap();
        match letter {
            "N" => wp_up += number,
            "S" => wp_up -= number,
            "E" => wp_right += number,
            "W" => wp_right -= number,
            "L" => rotate(&mut wp_up, &mut wp_right, number),
            "R" => rotate(&mut wp_up, &mut wp_right, 360 - number),
            "F" => {
                up += number * wp_up;
                right += number * wp_right;
            }
            _ => panic!(),
        }
    }
    println!("answer: {}", up.abs() + right.abs());
}

fn rotate(up: &mut isize, right: &mut isize, rot: isize) {
    let new_up = sin(rot) * *right + cos(rot) * *up;
    let new_right = cos(rot) * *right + -sin(rot) * *up;
    *up = new_up;
    *right = new_right;
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
