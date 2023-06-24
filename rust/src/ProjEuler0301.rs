fn main() {
    let mut count = 0;
    for f in 1..=1_073_741_824 {
        if 0 == f ^ (2 * f) ^ (3 * f) {
            count += 1;
        }
    }
    dbg!(count);
}
