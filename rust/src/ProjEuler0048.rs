fn main() {
    dbg!((1..=1000)
        .fold(0, |res, f| res + mod_pow(f, f))
        .rem_euclid(DIGITS));
}
const DIGITS: u64 = 10_u64.pow(10);

fn mod_pow(a: u64, b: u64) -> u64 {
    (0..b)
        .fold(1, |res, _| (res * a).rem_euclid(DIGITS))
        .rem_euclid(DIGITS)
}
