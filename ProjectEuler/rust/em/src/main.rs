#[macro_use]
extern crate timeit;

fn benchmark() {
    println!("----- Numeric functions: -----");
    print!("em.lb: ");
    timeit!({
        em::int64::integer::lb(1000000);
    });
    print!("em.log: ");
    timeit!({
        em::int64::integer::log(100000000, 8);
    });
}

fn main() {
    benchmark();
}