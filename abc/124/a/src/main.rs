// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }

    let mut ans = 0;
    for _ in 0..2 {
        if a > b {
            ans += a;
            a -= 1;
        } else {
            ans += b;
            b -= 1;
        }
    }
    println!("{}", ans);
}
