use machine_prime::is_prime;
fn main() {
    let mut count = 0;
    let count = (1..)
        .map(|a| 3 * a * a + 3 * a + 1)
        .take_while(|&to_check| to_check <= 1_000_000)
        .filter(|&to_check| is_prime(to_check))
        .count();
    print!("count: {}", count);
}

fn is_cubic(n: u64) -> bool {
    let n_f = n as f64;
    let cbrt = n_f.cbrt();
    (cbrt.fract()) < 1e-10
}
