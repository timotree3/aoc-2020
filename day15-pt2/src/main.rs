use std::collections::HashMap;

fn main() {
    let mut ages: HashMap<usize, usize> = [14, 8, 16, 0, 1, 17]
        .iter()
        .enumerate()
        .map(|(era, &num)| (num, era))
        .collect();
    let mut difference = 0;
    let mut t = ages.len();
    let mut last_spoken = difference;
    while t < (30000000 - 1) {
        let new = difference;
        difference = t - ages.get(&new).unwrap_or(&t);
        ages.insert(new, t);
        t += 1;
        last_spoken = difference;
    }
    println!("answer: {}", last_spoken);
}
