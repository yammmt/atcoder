// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut n: u32,
        k: u32,
    }

    let mut ans = 0;
    while n > 0 {
        n /= k;
        ans += 1;
    }
    println!("{}", ans);
}
