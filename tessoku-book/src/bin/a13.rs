use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        K:  usize,
        A: [usize; N]
    };

    let mut right = 0;
    let mut answer = 0;
    for left in 0..N - 1 {
        while right + 1 < N && A[right + 1] - A[left] <= K {
            right += 1
        }
        answer += right - left;
    }

    println!("{}", answer);
}
