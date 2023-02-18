use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: isize,
        K: isize
    }

    let mut choice = 0;
    for i in 1..=N {
        for j in 1..=N {
            let val = K - i - j;
            if 1 <= val && val <= N {
                choice += 1;
            }
        }
    }

    println!("{}", choice);
}
