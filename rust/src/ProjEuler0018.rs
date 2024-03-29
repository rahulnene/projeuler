use std::fs;
fn main() {
    let pyramid_str = fs::read_to_string("p018_triangle.txt").expect("File not found");
    let mut pyramid: Vec<Vec<u32>> = Vec::new();
    for line in pyramid_str.lines() {
        pyramid.push(
            line.split_whitespace()
                .map(|s| s.parse().expect("Failed to parse number"))
                .collect(),
        );
    }
    while pyramid.len() > 1 {
        let last_row = pyramid.pop().unwrap();
        let mut bottom_row = pyramid.pop().unwrap();
        for index in 0..bottom_row.len() {
            bottom_row[index] += last_row[index].max(last_row[index+1]);
        }
        pyramid.push(bottom_row);
    }
    dbg!(&pyramid[0]);
}
