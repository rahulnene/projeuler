fn main() {
    let a = (21..=40).fold(1_u128, |res, n| res * n);
    let b = (2..=20).fold(1_u128, |res,n| res*n);
    dbg!(a/b);
}
