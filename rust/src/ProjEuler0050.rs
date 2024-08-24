use std::time::Instant;

use machine_prime::{self, is_prime};

fn main() {
    let max = 1_000_000;
    let start = Instant::now();
    let primes = (2..max)
        .filter(|t| machine_prime::is_prime(*t))
        .collect::<Vec<_>>();
    let mut prime_sums: Vec<u64> = Vec::with_capacity(primes.len());
    prime_sums.push(0);
    let mut temp = 0;
    for p in primes {
        temp += p;
        if temp > max {
            break;
        }
        prime_sums.push(temp);
    }
    let mut all_prime_sums = Vec::new();
    for i in 0..prime_sums.len() {
        let a = unsafe { prime_sums.get_unchecked(i) };
        let b = prime_sums.last().unwrap() - a;
        if is_prime(*a) {
            all_prime_sums.push((*a, i));
        }
        if is_prime(b) {
            all_prime_sums.push((b, prime_sums.len() - i - 1));
        }
    }
    println!(
        "{:?}",
        all_prime_sums.iter().max_by_key(|(_, b)| b).unwrap()
    );
    dbg!(start.elapsed());
}
