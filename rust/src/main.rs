use core::cmp::Ordering;
use rayon::prelude::*;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const WINDOW_SIZE: usize = 100;

/// Given a slice of `numbers` and a `target` value, returns
/// true if there are two numbers in the slice that add up to the target value,
/// false otherwise.
///
/// # Examples
///
/// ```
/// let numbers = [1, 2, 3, 4, 5];
/// let target = 9;
///
/// assert_eq!(two_sum(&numbers, target), true);
/// ```
/// We use a 2 pointer method as it is the most efficient way to solve this problem.
fn two_sum(numbers: &[u128], target: u128) -> bool {
    let mut numbers: Vec<u128> = numbers.to_owned();
    numbers.sort_unstable();
    let (mut l, mut r): (u128, u128) = (0, numbers.len() as u128 - 1);
    while l < r {
        let cursum = numbers[l as usize] + numbers[r as usize];
        match cursum.cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => l += 1,
            Ordering::Greater => r -= 1,
        };
    }
    false
}

/// The program reads a list of numbers from a file,
///  finds all pairs of numbers in a sliding window of size `WINDOW_SIZE` that do not add up to the next number in the list,
///  and prints the numbers that violate this condition.
fn main() {
    let nums: Vec<u128> = BufReader::new(File::open("challenge_input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let ans: Vec<u128> = nums
        .par_windows(WINDOW_SIZE + 1)
        .filter(|window| !two_sum(window, window[WINDOW_SIZE]))
        .map(|window| window[WINDOW_SIZE])
        .collect();
    println!("{:?}", ans);
    println!("{}", ans.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        assert_eq!(two_sum(&[1, 2, 33, 54, 7].to_vec(), 35), true);
    }
}
