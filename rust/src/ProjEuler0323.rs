fn main() {
    let mut half_power: f64 = 4.0;
    let p: f64 = (0..50).fold(0.0, |res, i| {
        half_power *= 0.5;
        (f64::from(i)).mul_add((1.0 - half_power / 2.0).powi(32) - (1.0 - half_power).powi(32), res)
    });
    dbg!(p);
}
