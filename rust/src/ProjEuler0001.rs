fn main() {
    let ans:u32 = (1..1000).filter(|s| s%3 == 0 || s%5 == 0).sum();
    println!("{ans}");
}