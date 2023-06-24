use std::{
    fs::File,
    io::{BufRead,BufReader},
};

fn compute(base: f64, exp: f64) -> f64 {
    exp * f64::log10(base)
}

fn main() {
    let input = File::open("p099_base_exp.txt").expect("Failed to open file");
    let buffered = BufReader::new(input);
    let mut largest: (f64, i32) = (0.0, 0);
    let mut line_num = 1;
    let mut current: f64;
    for line in buffered.lines() {
        let line_vals: Vec<f64> = line
            .expect("Failed to read line")
            .split(',')
            .map(|f| f.parse::<f64>().expect("Failed to parse float"))
            .collect();
        current = compute(line_vals[0], line_vals[1]);
        if current > largest.0 {
            largest = (current, line_num);
        }
        line_num += 1;
    }
    dbg!(largest.1);
}
