use regex::RegexSet;
use std::io::Read;

fn main() {
    #[allow(clippy::trivial_regex)]
    let pattern = RegexSet::new(&["cid", "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]).unwrap();
    let mut data = String::new();
    std::io::stdin().read_to_string(&mut data).unwrap();
    let passports = data.split("\n\n");
    let mut count: u64 = 0;
    for passport in passports {
        dbg!(&passport);
        let matches = pattern.matches(&passport);
        let required_fields = if matches.matched(0) { 8 } else { 7 };
        if dbg!(matches.iter().count()) == dbg!(required_fields) {
            count += 1;
        }
    }

    println!("answer: {}", count);
}
