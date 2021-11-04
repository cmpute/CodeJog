use em::u64::PrimeBuffer;

const TARGET: u64 = 10001;

fn solve(target: Option<u64>) -> u64 {
    let mut pb = PrimeBuffer::new();
    let p = pb.nprimes(target.unwrap_or(TARGET) as usize).last();
    *p.unwrap()
}

fn main() {
    println!("{}", solve(None))
}
