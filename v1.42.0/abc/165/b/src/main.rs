// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let mut money = 100;
    let mut ans = 0;
    while x > money {
        money += money / 100;
        ans += 1;
    }
    println!("{}", ans);
}
