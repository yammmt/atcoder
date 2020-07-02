// -*- coding:utf-8-unix -*-

use proconio::input;

// ABC086C - Traveling
// https://atcoder.jp/contests/abs/fasks/arc089_a

fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }

    let mut ans = 0;
    while a.iter().all(|x| x % 2 == 0) {
        ans += 1;
        a = a.iter().map(|x| x / 2).collect();
    }
    println!("{}", ans);
}
