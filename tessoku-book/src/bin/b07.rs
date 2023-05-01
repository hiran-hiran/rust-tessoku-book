use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        T: usize,
        N: usize,
        LR: [(usize, usize); N]
    };

    let mut B = vec![0; T + 1];
    // println!("B: {:?}", B);

    for &(L, R) in &LR {
        B[L] += 1;
        B[R] -= 1;
    }

    let mut answer = 0;
    for b in &B[..T] {
        answer += b;
        println!("{}", answer);
    }

    // println!("B: {:?}", B);
    // println!("B[1]: {:?}", B[1]);

    // println!("T: {}", T);
    // println!("N: {}", N);
    // println!("LR: {:?}", LR);
}
