fn nth_prime(n: i64) -> i64 {
    let mut is_prime = vec![true; (n + 1) as usize];
    let mut sum = 0;
    for i in 2..=n {
        if is_prime[i as usize] {
            sum += i;
            (i * i..=n)
                .step_by(i as usize)
                .for_each(|j| is_prime[j as usize] = false);
        }
    }
    sum
}

fn main() {
    println!("{}", nth_prime(2000001));
}
