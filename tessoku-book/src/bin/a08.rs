use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        H: usize,
        W: usize,
        matrix: [[usize; W]; H],
        Q: usize,
        ABCD : [(usize, usize, usize, usize); Q]
    };

    let mut cum = vec![vec![0; W + 1]; H + 1];

    for h in 1..=H {
        for w in 1..=W {
            cum[h][w] = cum[h][w - 1] + matrix[h - 1][w - 1];
        }
    }
    for w in 1..=W {
        for h in 1..=H {
            cum[h][w] += cum[h - 1][w];
        }
    }

    for &(A, B, C, D) in &ABCD {
        let answer = cum[C][D] + cum[A - 1][B - 1] - cum[C][B - 1] - cum[A - 1][D];

        println!("{}", answer);
    }
}
