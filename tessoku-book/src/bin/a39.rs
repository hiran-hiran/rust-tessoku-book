use proconio::{fastout, input};

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        mut LR: [(usize, usize); N]
    }

    let mut current_time = 0;
    let mut answer = 0;
    // 終わるのが早い順にsort
    LR.sort_by(|l, r| l.1.cmp(&r.1));
    // imutable ver.
    // let sorted_LR = LR.iter().cloned().sorted_by(|l, r| l.1.cmp(&r.1));

    for (l, r) in LR {
        if current_time <= l {
            current_time = r;
            answer += 1;
        }
    }

    println!("{}", answer);
}
