fn main() {
    let a = (2..=10000).filter(|i| is_odd(continued_fraction(*i).len() as u64)).count();
    dbg!(a);
}

fn continued_fraction(n: u64) -> Vec<u64> {
    if is_square(n) {
        return Vec::new();
    }
    let mut result = vec![];
    let a0 = (n as f64).sqrt() as u64;
    let mut m = 0;
    let mut d = 1;
    let mut a = a0;

    while a != 2 * a0 {
        m = d * a - m;
        d = (n - m * m) / d;
        a = (a0 + m) / d;
        result.push(a);
    }

    result
}

const fn is_odd(n: u64) -> bool {
    n % 2 == 1
}

fn is_square(n: u64) -> bool {
    let root = (n as f64).sqrt() as u64;
    root * root == n
}