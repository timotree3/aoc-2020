use std::io::Read;

const EMPTY: u8 = b'L';
const FULL: u8 = b'#';

fn main() {
    let mut data = Vec::new();
    std::io::stdin().read_to_end(&mut data).unwrap();
    let width = data.iter().position(|&b| b == b'\n').unwrap();
    let line_length = width + 1;
    let mut scratch = Vec::with_capacity(data.len());
    loop {
        println!("\n\n\n{}", std::str::from_utf8(&data).unwrap());
        scratch.clear();
        scratch.extend(
            data.iter()
                .enumerate()
                .map(|(i, &value)| new_state(&data, i, value, line_length)),
        );
        if data == scratch {
            break;
        }
        std::mem::swap(&mut data, &mut scratch);
    }
    println!("answer: {}", data.iter().filter(|&&b| b == FULL).count());
}

fn new_state(data: &[u8], index: usize, value: u8, line_length: usize) -> u8 {
    match value {
        EMPTY if count_neighbors_full(data, index, line_length) == 0 => FULL,
        FULL if count_neighbors_full(data, index, line_length) >= 5 => EMPTY,
        state => state,
    }
}

fn see_full_in_direction(
    data: &[u8],
    start: usize,
    line_length: usize,
    down: isize,
    right: isize,
) -> bool {
    let mut index = start as isize;
    let offset = right + (line_length as isize * down);
    loop {
        index += offset;
        // dbg!(
        //     index,
        //     down,
        //     right,
        //     offset,
        //     data.get(index as usize)
        //         .map(|&b| char::try_from(b).unwrap())
        // );
        match data.get(index as usize) {
            Some(&FULL) => return true,
            Some(&EMPTY) => return false,
            Some(&b'\n') => return false,
            Some(_) => {}
            None => return false,
        }
    }
}

fn count_neighbors_full(data: &[u8], index: usize, line_length: usize) -> usize {
    (-1..2)
        .flat_map(|down| (-1..2).map(move |right| (down, right)))
        .filter(|&(down, right)| {
            !(down == 0 && right == 0)
                && see_full_in_direction(data, index, line_length, down, right)
        })
        .count()
}
