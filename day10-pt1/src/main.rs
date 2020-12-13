use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let mut data: Vec<u32> = handle
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    data.push(0);
    data.sort_unstable();
    let gaps_of_1 = data
        .windows(2)
        .map(|window| window[1] - window[0])
        .filter(|&gap| gap == 1)
        .count();
    let gaps_of_3 = data
        .windows(2)
        .map(|window| window[1] - window[0])
        .filter(|&gap| gap == 3)
        .count()
        + 1;

    println!("answer: {}", gaps_of_1 * gaps_of_3,);
}
