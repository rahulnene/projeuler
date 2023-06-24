fn main() {
    let (mut a, mut b) = (1, 2);
    let mut c: i32;
    let mut sum = 0;
    while b < 40_000_000 {
        c = fibb(a, b);
        (a, b) = (b, c);
        if c % 2 == 0 {
            sum += c;
        }
    }

    println!("{}", sum + 2);
}

const fn fibb(prev1: i32, prev2: i32) -> i32 {
    prev1 + prev2
}
