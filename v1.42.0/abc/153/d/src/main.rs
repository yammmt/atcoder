// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut h: u64,
    }

    let mut twon = 1u64;
    let mut ans = 0u64;
    while h > 0 {
        h /= 2;
        ans += twon;
        twon *= 2;
    }
    println!("{}", ans);
}
