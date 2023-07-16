use std::time::Instant;

fn main() {
    
    const A:u64 = 1_504_170_715_041_707;
    const MOD:u64 = 4_503_599_627_370_517;
    let start = Instant::now();

    let (mut total, mut min, mut max) = (A,A,A);

    while min > 1 {
        let temp = (min + max) % MOD;
        if temp > max {
            max = temp;
        }
        if temp < min {
            min = temp;
            total += min;
        }
    }

    println!("{total}", );
    println!("{:?}", start.elapsed());
}