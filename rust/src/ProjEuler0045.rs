use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut n = 1;
    let mut count = 0;

    while count <= 4 {
        let hex = n * (2 * n - 1);
        if is_pentagonal(hex) {
            println!("{hex}");
            count += 1;
        }
        n += 1;
    }

    println!("Time elapsed: {:?}", start.elapsed());
}

fn is_pentagonal(x: u64) -> bool {
    let det = (1 + 24 * x) as f64;
    let sqrt = (det.sqrt() + 1.0) / 6.0;
    sqrt.fract() == 0.0
}
