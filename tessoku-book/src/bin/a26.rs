use num_integer::Roots;
use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        Q: usize,
        X: [usize; Q]
    }

    fn is_prime(x: &usize) -> &str {
        let mut answer = "Yes";

        for r in 2..=x.sqrt() {
            if x % r == 0 {
                answer = "No";
            }
        }

        return answer;
    }

    for &x in &X {
        let result = is_prime(&x);
        println!("{}", result);
    }
}
