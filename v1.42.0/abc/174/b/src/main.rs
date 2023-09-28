// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        d: i64,
    }

    let distance = d * d;
    let mut ans = 0;
    for _i in 0..n {
        input! {
            x: i64,
            y: i64,
        }
        if x * x + y * y <= distance {
            ans += 1;
        }
    }
    println!("{}", ans);
}
