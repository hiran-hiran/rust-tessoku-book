use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        M: usize,
        B: usize,
        A: [usize; N],
        C: [usize; M],
    }

    let calc_b = B * N * M;
    let calc_a = A.iter().sum::<usize>() * M;
    let calc_c = C.iter().sum::<usize>() * N;

    println!("{}", calc_a + calc_b + calc_c);
}
