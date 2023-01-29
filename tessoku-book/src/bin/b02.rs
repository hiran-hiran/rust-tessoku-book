use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        A: u16,
        B: u16,
    }

    let mut result: &str = "No";

    for num in A..=B {
        if 100 % num == 0 {
            result = "Yes";
        }
    }

    println!("{}", result);
}
