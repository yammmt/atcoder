// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    let mut money = 100.0f64;
    let mut ans = 0;
    while x > money {
        // println!("{}", money);
        money = (money * 1.01).floor();
        ans += 1;
    }
    println!("{}", ans);
}
