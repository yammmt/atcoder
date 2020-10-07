// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u32,
    }

    let mut ans = 0;
    let a = x / 500;
    ans += 1000 * a;
    let b = (x - 500 * a) / 5;
    ans += 5 * b;
    println!("{}", ans);
}
