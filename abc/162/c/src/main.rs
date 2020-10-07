// -*- coding:utf-8-unix -*-

use proconio::input;

fn gcd(a: u8, b: u8) -> u8 {
    if a % b == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    input! {
        k: u8,
    }

    let mut ans: u32 = 0;
    for a in 1..k + 1 {
        for b in a..k + 1 {
            for c in b..k + 1 {
                if a == b && b == c {
                    ans += gcd(gcd(a, b), c) as u32;
                } else if a == b || b == c {
                    ans += 3 * gcd(gcd(a, b), c) as u32;
                } else {
                    ans += 6 * gcd(gcd(a, b), c) as u32;
                }
                // println!("{}, {}, {}: {}", a, b, c, gcd(gcd(a, b), c));
            }
        }
    }
    println!("{}", ans);
}
