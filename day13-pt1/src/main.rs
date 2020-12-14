use std::io::Read;

fn main() {
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let mut lines = data.lines();
    let earliest: usize = lines.next().unwrap().parse().unwrap();
    let (id, wait_time) = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| id.parse().unwrap())
        .map(|id: usize| dbg!(id, id - (earliest % id)))
        .min_by_key(|&(_id, wait_time)| wait_time)
        .unwrap();
    println!("answer: {}", id * wait_time);
}
