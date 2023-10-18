use const_primes::Primes;

// #[allow(long_running_const_eval)]
const PRIMES: Primes<1000> = Primes::new();

fn main() {
    let mut max = 0;
    let mut max_ab = (0, 0);
    for a in -999..1000 {
        for b in -1000..=1000 {
            let ans = eval(a, b);
            if ans > max {
                max = ans;
                max_ab = (a, b);
            }
        }
    }
    dbg!(max, max_ab, max_ab.0 * max_ab.1);
}

fn eval(a: i32, b: i32) -> i32 {
    let mut n = 0;
    let f = move |n| n * n + a * n + b;
    while f(n) > 0 && PRIMES.is_prime(f(n) as u32).unwrap() {
        n += 1;
    }
    n
}
