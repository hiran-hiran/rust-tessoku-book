use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        D: usize,
        N: usize,
        LRH: [(usize, usize, usize); N]
    }

    let mut L = vec![24; D + 1];

    for &(l, r, h) in &LRH {
        for d in l..=r {
            L[d] = L[d].min(h);
        }
    }

    let ans: usize = L[1..].iter().sum();
    println!("{}", ans);
}
