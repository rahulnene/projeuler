const DIGITS: u64 = 10_u64.pow(8);

fn mod_pow(a: u64, b: u64) -> u64 {
    (0..b)
        .fold(1, |res, _| (res * a) % DIGITS)
        .rem_euclid(DIGITS)
}

fn mod_hyperpow(a: u64, k: u64) -> u64 {
    if k == 1 {
        return a;
    }
    mod_pow(a, mod_hyperpow(a, k - 1))
}

fn main() {
    dbg!(mod_hyperpow(1777, 9));
}
