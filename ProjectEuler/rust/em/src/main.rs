#[macro_use]
extern crate timeit;

fn benchmark() {
    println!("----- Numeric functions: -----");
    print!("em.lb: ");
    timeit!({
        em::u64::lb(1000000);
    });
    print!("em.log: ");
    timeit!({
        em::u64::log(100000000, 8);
    });
}

fn main() {
    benchmark();
}