use proconio::{fastout, input};
// use superslice::Ext;

#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        N: usize,
        X: usize,
        A: [usize; N]
    }

    let answer = A.binary_search(&X).unwrap() + 1;

    // let answer = binary_search(A, X).unwrap() + 1;
    println!("{:?}", answer);
}

// fn binary_search(a: Vec<usize>, x: usize) -> Option<usize> {
//     let mut left = 0;
//     let mut right = a.len() - 1;

//     while left <= right {
//         let center = (left + right) / 2;

//         if x < a[center] {
//             right = center - 1;
//         }
//         if x == a[center] {
//             return Some(center);
//         }
//         if x > a[center] {
//             left = center + 1;
//         }
//     }

//     return None;
// }
