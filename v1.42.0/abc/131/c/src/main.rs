// -*- coding:utf-8-unix -*-

// 21min. 1WA

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
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }

    let cn = b / c - (a - 1) / c;
    let dn = b / d - (a - 1) / d;
    let lcm = c / gcd(c, d) * d;
    let cdn = b / lcm - (a - 1) / lcm;
    println!("{}", (b - a + 1) - cn - dn + cdn);
}
