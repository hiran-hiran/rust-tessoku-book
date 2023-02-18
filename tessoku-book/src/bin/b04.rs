use proconio::{fastout, input, marker::Chars};
fn main() {
    input! {
        N: Chars
    }

    let mut res = 0;

    for (i, &c) in N.iter().rev().enumerate() {
        if c == '1' {
            res += 1 << i;
        }
    }

    println!("{}", res);
}
