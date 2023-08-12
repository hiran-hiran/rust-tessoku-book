use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K: usize
    }

    let isMinTimes = K >= N * 2 - 2;
    let isEven = K % 2 == 0;

    if isMinTimes && isEven {
        println!("Yes");
    } else {
        println!("No");
    }
}
