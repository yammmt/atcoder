// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        m: u64,
        mut a: [u64; n],
    }

    a.sort();
    a.reverse();
    let sum: u64 = a.iter().sum();
    let ans = if a[(m - 1) as usize] >= sum / (4 * m) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
