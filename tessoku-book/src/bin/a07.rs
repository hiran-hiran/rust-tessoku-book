use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        D: usize,
        N: usize,
        LR: [(usize,usize); N]
    }

    let mut B = vec![0; D + 1];

    for &(L, R) in &LR {
        B[L - 1] += 1;
        B[R] -= 1;
    }

    let mut answer = 0;
    for b in &B[..D] {
        answer += b;
        println!("{:?}", answer);
    }
}
