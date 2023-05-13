use proconio::{fastout, input};
use std::cmp;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        H: [isize; N],
    }

    let mut dp = vec![0; N + 1];
    dp[2] = (H[1] - H[0]).abs();

    for i in 3..=N {
        let h1 = (H[i - 1] - H[i - 2]).abs() + dp[i - 1];
        let h2 = (H[i - 1] - H[i - 3]).abs() + dp[i - 2];
        dp[i] = cmp::min(h1, h2);
        //     let a = dp[i - 1] + A[i - 2];
        //     let b = dp[i - 2] + B[i - 3];
        //     dp[i] = cmp::min(a, b)
    }

    println!("{:?}", dp[N]);
}
