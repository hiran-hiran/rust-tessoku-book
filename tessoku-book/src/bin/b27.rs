use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut A: usize,
        mut B: usize,
    }

    let gcd = get_gcd(A, B);
    let result = A * B / gcd;

    println!("{}", result);
}

fn get_gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else {
        get_gcd(b % a, a)
    }
}
