use std::collections::HashSet;

const LIMIT: u32 = 1000;

fn solve(limit: Option<u32>) -> u32 {
    let limit = limit.unwrap_or(LIMIT);
    let mut mark = HashSet::new();
    let mut three = 0;
    let mut five = 0;
    let mut total = 0;
    while three < limit {
        total += three;
        mark.insert(three);
        three += 3;
    }
    while five < limit {
        if !mark.contains(&five) {
            total += five;
        }
        five += 5;
    }
    total
}

fn main() {
    println!("{}", solve(None));
}
