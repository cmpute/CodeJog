use em::traits::Integer;

const LIMIT: u64 = 20;

fn solve(limit: Option<u64>) -> u64 {
    let limit = limit.unwrap_or(LIMIT);

    let mut total = 1;
    for i in 2..limit + 1 {
        total = total * i / total.gcd(&i);
    }
    total
}

fn main() {
    println!("{}", solve(None));
}
