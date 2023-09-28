// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: u64,
        c: u64,
    }

    if s * 2 > c {
        // c がボトルネック
        println!("{}", c / 2);
    } else {
        // s 全部使う
        let mut ans = s;
        let c_left = c - 2 * s;
        ans += c_left / 4;
        println!("{}", ans);
    }
}
