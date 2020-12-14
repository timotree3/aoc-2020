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
        FULL if count_neighbors_full(data, index, line_length) >= 4 => EMPTY,
        state => state,
    }
}

fn count_neighbors_full(data: &[u8], index: usize, line_length: usize) -> u32 {
    let x = index % line_length;
    let width = line_length - 1;
    let mut count = 0;
    if index >= line_length && x >= 1 && is_full(data, index - 1 - line_length) {
        count += 1;
    }
    if index >= line_length && is_full(data, index - line_length) {
        count += 1;
    }
    if index >= line_length && x < width - 1 && is_full(data, index + 1 - line_length) {
        count += 1;
    }
    if x >= 1 && is_full(data, index - 1) {
        count += 1;
    }
    if x < width - 1 && is_full(data, index + 1) {
        count += 1;
    }
    if index <= data.len() - 2 - line_length && x >= 1 && is_full(data, index + line_length - 1) {
        count += 1;
    }
    if index <= data.len() - 2 - line_length && is_full(data, index + line_length) {
        count += 1;
    }
    if index <= data.len() - 2 - line_length
        && x < width - 1
        && is_full(data, index + line_length + 1)
    {
        count += 1;
    }
    count
}

fn is_full(data: &[u8], index: usize) -> bool {
    data[index] == FULL
}
