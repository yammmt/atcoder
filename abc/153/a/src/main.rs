// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: u32,
        a: u32,
    }
    let mut ans = h / a;
    if h % a != 0 {
        ans += 1;
    }
    println!("{}", ans);
}
