const DIGITS: u64 = 10_u64.pow(10);
fn mod_pow(a: u64, b: u64) -> u64 {
    (0..b)
        .fold(1, |res, _| (res * a) % DIGITS)
        .rem_euclid(DIGITS)
}
fn main() {
    println!("{}", (mod_pow(2, 7830457) * 28433 + 1) % DIGITS);
}
