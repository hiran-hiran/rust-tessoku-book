use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        TA: [(String, isize); N]
    }

    let mut num: isize = 0;

    for (T, A) in &TA {
        match T.as_str() {
            "+" => num += A,
            "-" => num -= A,
            "*" => num *= A,
            _ => panic!(),
        }

        num %= 10000;
        if num < 0 {
            num += 10000;
        }

        println!("{}", num);
    }
}
