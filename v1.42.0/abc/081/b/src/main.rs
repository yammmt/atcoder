// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut ans = 0;
    while a.iter().all(|&b| b % 2 == 0) {
        ans += 1;
        a = a.iter().map(|b| b / 2).collect();
    }
    println!("{}", ans);
}
