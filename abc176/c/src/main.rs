// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    let mut ans = 0;
    for i in 1..a.len() {
        if a[i] < a[i - 1] {
            ans += a[i - 1] - a[i];
            a[i] = a[i - 1];
        }
    }
    println!("{}", ans);
}
