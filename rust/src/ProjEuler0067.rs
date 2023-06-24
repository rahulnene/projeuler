use std::fs;
fn main() {
    let pyramid_str = fs::read_to_string("p067_triangle.txt").expect("Unable to read file to string");
    let mut pyramid: Vec<Vec<u32>> = Vec::new();
    for line in pyramid_str.lines() {
        pyramid.push(
            line.split_whitespace()
                .map(|s| s.parse().expect("Unable to parse pyramid string to u32"))
                .collect(),
        );
    }
    while pyramid.len() > 1 {
        let last_row = pyramid.pop().expect("Unable to pop last row");
        let mut bottom_row = pyramid.pop().expect("Unable to pop bottom row");
        for index in 0..bottom_row.len() {
            bottom_row[index] += last_row[index].max(last_row[index+1]);
        }
        pyramid.push(bottom_row);
    }
    dbg!(pyramid);
}
