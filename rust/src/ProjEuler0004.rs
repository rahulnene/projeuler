fn main() {
    const RANGE: i32 = 1000;
    let start = std::time::Instant::now();
    let mut largest = 0;
    for a in 100..RANGE {
        for b in 100..RANGE {
            let product = a * b;
            if is_palindrome(product) && product > largest {
                largest = product;
            }
        }
    }
    println!("{largest}", );
    println!("{:?}", start.elapsed());
}

fn reverse(inp: &str) -> String {
    inp.chars().rev().collect::<String>()
}

fn is_palindrome(check: i32) -> bool {
    let check_string = check.to_string();
    check_string == reverse(&check_string)
}
