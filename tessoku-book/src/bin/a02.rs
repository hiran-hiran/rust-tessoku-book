use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: u16,
        X: u16,
        A: [u16; N],
    }

    let mut result: &str = "No";

    for a in A {
        if a == X {
            result = "Yes"
        }
    }

    println!("{}", result);
}
