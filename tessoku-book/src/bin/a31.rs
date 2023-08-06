use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N:usize
    }

    let n3 = N / 3;
    let n5 = N / 5;
    let n15 = N / 15;

    let result = n3 + n5 - n15;

    println!("{}", result);
}
