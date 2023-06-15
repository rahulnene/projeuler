use divisors::get_divisors;
use primal::{self, Sieve};
use rayon::prelude::*;

const MAX: usize = 100_000_000;

fn main() {
    let primes_cache = primal::Sieve::new(MAX);
    let ans = 3 + (1..MAX / 4)
    .into_par_iter()
    .filter_map(|k| {
        let n = 4 * k + 2;
        let n_plus_1 = n + 1;
        let n_div_2_plus_2 = n / 2 + 2;
        
        if primal::Sieve::is_prime(&primes_cache, n_plus_1)
            && primal::Sieve::is_prime(&primes_cache, n_div_2_plus_2)
        {
            Some(Number::new(n as u64).check(&primes_cache) as u64 * n as u64)
        } else {
            None
        }
    })
    .fold(|| 0, |res, num| res + num)
    .sum::<u64>();
    dbg!(ans);
}

#[derive(Debug)]
struct Number {
    value: u64,
}

fn generate_divisors(number: u64) -> Vec<u64> {
    let mut divisors = get_divisors(number);
    divisors.push(1);
    divisors.push(number);
    divisors
}

impl Number {
    fn new(value: u64) -> Self {
        Number { value }
    }
    fn check(&self, prime_list: &Sieve) -> bool {
        let divisors = generate_divisors(self.value);
        let mut divisors_iter = divisors.iter();
        let mut check_flag = true;
        while check_flag {
            let d = divisors_iter.next();
            match d {
                None => {
                    return true;
                }
                Some(div) => {
                    check_flag =
                        primal::Sieve::is_prime(&prime_list, (div + self.value / div) as usize)
                }
            }
        }
        false
    }
}
