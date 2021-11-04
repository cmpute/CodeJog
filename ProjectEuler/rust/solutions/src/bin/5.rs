use em::u64::gcd;

const LIMIT: u64 = 20;

fn solve(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(LIMIT);

    let mut total = 1;
    for i in 2..limit + 1 {
        total = total * i / gcd(total, i);
    }
    total
}

fn main() {
    println!("{}", solve(None));
}
