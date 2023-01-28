use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u16,
        b: u16
    }

    println!("{}", a + b);
}
