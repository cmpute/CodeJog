use em::int64::prime::PrimeBuffer;

const TARGET: u64 = 600851475143;

fn solve(target: Option<u64>) -> u64 {
    let mut pb = PrimeBuffer::new();
    let facs = pb.factors(target.unwrap_or(TARGET));
    *facs.keys().max().unwrap()
}

fn main() {
    println!("{}", solve(None));
}
