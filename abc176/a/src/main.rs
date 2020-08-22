// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        x: u32,
        t: u32,
    }
    let mut ans = (n / x) * t;
    if n % x != 0 {
        ans += t;
    }
    println!("{}", ans);
}
