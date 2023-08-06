use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N:usize
    }

    let n3 = N / 3;
    let n5 = N / 5;
    let n7 = N / 7;
    let n15 = N / 15;
    let n21 = N / 21;
    let n35 = N / 35;
    let n105 = N / 105;

    let result = n3 + n5 + n7 - n15 - n21 - n35 + n105;

    println!("{}", result);
}
