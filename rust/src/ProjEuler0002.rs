fn main() {
    let (mut a, mut b) = (1, 2);
    let mut c: i32;
    let mut sum = 0;
    while b < 40_000_000 {
        c = a+b;
        (a, b) = (b, c);
        if c % 2 == 0 {
            sum += c;
        }
    }

    println!("{}", sum + 2);
}
