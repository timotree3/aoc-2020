use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut pairs: Vec<(usize, usize)> = data
        .lines()
        .nth(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|&(_, id)| id != "x")
        .map(|(i, id)| (i, id.parse::<usize>().unwrap()))
        .collect();
    pairs.sort_unstable_by_key(|&(_i, id)| id);
    let mut t = 0;
    let mut step = 1;
    for &(i, id) in pairs.iter() {
        while (t + i) % id != 0 {
            t += step;
        }
        step *= id;
    }
    println!("answer: {}", t);
}
