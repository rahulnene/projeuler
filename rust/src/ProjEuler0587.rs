use std::time::Instant;
use std::f32::consts::FRAC_PI_4;

fn main() {
    let start = Instant::now();

    let end = 0.1;

    let result = (1000..).find(|&n: &i32| {
        let x0 = n as f32 * (n as f32 + 1.0 - (2.0 * n as f32).sqrt()) / (n.pow(2) as f32 + 1.0);
        let area = x0 / (2.0 * n as f32);
        let angle = 1.0 - x0 - (1.0 - x0).asin();
        let total_area = area + (angle / 2.0);
        const L:f32 = 1.0 - FRAC_PI_4;
        100.0 * (total_area / L) < end
    });

    println!("{}", result.unwrap());

    println!("{:?}", start.elapsed());
}
