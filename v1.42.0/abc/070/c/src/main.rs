// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    // a * b / gcd(a, b)
    let d = gcd(a, b);
    a / d * b
}

fn main() {
    input! {
        n: u8,
        t: [u64; n],
    }

    let mut ans = t[0];
    for i in 1..n {
        ans = lcm(ans, t[i as usize]);
    }
    println!("{}", ans);
}
