// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = a[0];
    for i in 1..n {
        ans = gcd(ans, a[i]);
    }
    println!("{}", ans);
}
