// -*- coding:utf-8-unix -*-

// https://pyteyon.hatenablog.com/entry/2018/09/02/094228

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
    }

    let mut ans = 0;
    if k % 2 == 1 {
        ans = (n / k).pow(3);
    } else {
        ans = (n / k).pow(3) + ((n + k / 2) / k).pow(3);
    }
    println!("{}", ans);
}
