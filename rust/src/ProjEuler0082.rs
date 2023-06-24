use std::fs::read_to_string;

fn min_distance(
    matrix: &Vec<Vec<u32>>,
    mask: &Vec<Vec<u32>>,
    x: usize,
    y: usize,
    rows: usize,
) -> u32 {
    let mut lengths = Vec::new();
    let mut up_curr = 0;

    for up in (0..x).rev() {
        lengths.push(up_curr + matrix[up][y] + mask[up][y - 1]);
        up_curr += matrix[up][y];
    }

    let mut down_curr = 0;
    for down in (x + 1)..rows {
        lengths.push(down_curr + matrix[down][y] + mask[down][y - 1]);
        down_curr += matrix[down][y];
    }

    *lengths.iter().min().expect("Failed to find minimum")
}

fn compute(matrix: Vec<Vec<u32>>) -> u32 {
    let rows = matrix.len();
    let columns = matrix[0].len();

    let mut mask: Vec<Vec<u32>> = vec![vec![0; columns]; rows];

    for x in 0..rows {
        mask[x][0] = matrix[x][0];
    }

    for y in 1..columns {
        for x in 0..rows {
            mask[x][y] +=
                matrix[x][y] + mask[x][y - 1].min(min_distance(&matrix, &mask, x, y, rows));
        }
    }

    mask.iter().map(|row| row[columns - 1]).min().expect("Failed to find minimum")
}

fn read_matrix() -> Vec<Vec<u32>> {
    let pyramid_str = read_to_string("p082_matrix.txt").expect("Failed to read file");
    let mut pyramid: Vec<Vec<u32>> = Vec::new();
    for line in pyramid_str.lines() {
        pyramid.push(line.split(',').map(|s| s.parse().expect("Failed to parse as int")).collect());
    }
    pyramid
}

fn main() {
    println!("Minimum path sum: {}", compute(read_matrix()));
}
