const LIMIT: u32 = 4000000;

fn solve(limit: Option<u32>) -> u32 {
    let limit = limit.unwrap_or(LIMIT);
    let mut fib = vec![1, 1];
    let mut total = 1;
    while *(fib.last().unwrap()) < limit {
        if fib.last().unwrap() & 1 > 0 {
            total += fib.last().unwrap();
        }
        fib.push(fib.iter().rev().take(2).sum())
    }
    total
}

fn main() {
    println!("{}", solve(None));
}
