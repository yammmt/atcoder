// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        l: u16,
        r: u16,
        d: u16,
    }

    let mut ans = 0;
    for i in l..r + 1 {
        if i % d == 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
