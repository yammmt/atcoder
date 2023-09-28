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

    // 愚直に全通り加算してもオーダーは変わらず間に合う
    // 実行速度としては 55ms と 237ms くらい違う
    let mut ans: u32 = 0;
    for a in 1..k + 1 {
        for b in 1..k + 1 {
            for c in 1..k + 1 {
                let cur = gcd(gcd(a, b), c);
                if a == b && b == c {
                    ans += cur as u32;
                } else if a == b || b == c || c == a {
                    ans += 3 * cur as u32;
                } else {
                    ans += 6 * cur as u32;
                }
            }
        }
    }

    println!("{}", ans);
}
