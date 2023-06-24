const RANGE: usize = 80;

fn main() {
    let fib = Fibonacci::new().take(RANGE + 1).collect::<Vec<u64>>();
    let mut G: Vec<u64> = vec![1; RANGE + 1];
    for n in 3..=RANGE {
        G[n] = G[n - 1] + G[n - 2] + fib[n - 1];
    }
    dbg!(G[RANGE]);
}

#[derive(Debug)]
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    const fn new() -> Self {
        Self { a: 1, b: 0 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.b;
        self.b = self.a;
        self.a += r;
        Some(r)
    }
}
