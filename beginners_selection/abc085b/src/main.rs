// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u8,
        mut d: [u8; n],
    }

    d.sort();
    d.reverse();

    let mut ans = 1;
    for i in 0..d.len() - 1 {
        if d[i] > d[i + 1] {
            ans += 1;
        }
    }
    println!("{}", ans);
}
