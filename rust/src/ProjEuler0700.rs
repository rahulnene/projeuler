use std::{time::Instant};

fn main() {
    let start = Instant::now();
    const A:u64 = 1504170715041707;
    const MOD:u64 = 4503599627370517;

    let (mut total, mut min, mut max) = (A,A,A);

    while min > 1 {
        let temp = (min + max) % MOD;
        if temp > max {
            max = temp;
        }
        if temp < min {
            min = temp;
            total += min;
        }
    }

    println!("{}", total);
    println!("{:?}", start.elapsed());
}