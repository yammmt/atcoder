// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let d = 998244353;

    // 余り取ってから割り算すると死ぬ
    // (10^9)^2 < std::u64::MAX より掛けてから割って良い
    let asum = ((a * (a + 1)) / 2) % d;
    let bsum = ((b * (b + 1)) / 2) % d;
    let csum = ((c * (c + 1)) / 2) % d;

    let ans = (((asum * bsum) % d) * csum) % d;
    println!("{}", ans);
}
