use std::{cmp::Ordering, convert::TryInto, io::BufRead};

fn main() {
    let stdin = std::io::stdin();
    let handle = stdin.lock();
    let nums: Vec<u64> = handle
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut sorted_cache: [u64; 25] = nums[..25].try_into().unwrap();
    sorted_cache.sort_unstable();
    let mut i = 25;
    while is_sum_of_two(nums[i], &sorted_cache) {
        let old = nums[i - 25];
        let old_position = sorted_cache.iter().rposition(|&num| num == old).unwrap();
        sorted_cache[old_position] = nums[i];
        sorted_cache.sort_unstable();
        i += 1;
    }
    println!("answer: {}", nums[i]);
}

fn is_sum_of_two(num: u64, set: &[u64]) -> bool {
    let mut low = 0;
    let mut high = set.len() - 1;
    while low < high {
        match (set[low] + set[high]).cmp(&num) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Greater => {
                high -= 1;
            }
            Ordering::Less => {
                low += 1;
            }
        }
    }
    false
}
