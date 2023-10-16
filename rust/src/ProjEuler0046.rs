use const_primes::Primes;

// #[allow(long_running_const_eval)]
const PRIMES: Primes<10000> = Primes::new();

fn main() {
    for i in (3..).step_by(2) {
        if !PRIMES.is_prime(i).unwrap() && !goldbach_checker(i) {
            println!("{} is not a goldbach number", i);
            break;
        }
    }
}

fn goldbach_checker(n: u32) -> bool {
    let max_square = ((n / 2) as f64).sqrt() as u32;
    for i in 1..max_square + 1 {
        if PRIMES.is_prime(n - 2 * i * i).unwrap() {
            return true;
        }
    }
    false
}
