use proconio::{fastout, input};

fn main() {
    input! {
        N: i32
    }

    for x in (0..10).rev() {
        let wari = 1 << x;
        print!("{:?}", N / wari % 2);
    }

    println!();
}
