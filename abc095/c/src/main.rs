// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        x: u64,
        y: u64,
    }

    let ab_only = c * 2 * x.max(y);
    let no_ab = a * x + b * y;
    let ab_and = if x < y {
        c * 2 * x + b * (y - x)
    } else {
        c * 2 * y + a * (x - y)
    };
    println!("{}", ab_only.min(no_ab.min(ab_and)));

    // if a + b <= 2 * c {
    //     println!("{}", a * x + b * y);
    //     return;
    // }

    // if 2 * c <= a && 2 * c <= b {
    //     println!("{}", c * 2 * x.max(y));
    //     return;
    // }

    // if x < y {
    //     println!("{}", c * 2 * x + b * (y - x));
    // } else {
    //     println!("{}", c * 2 * y + a * (x - y));
    // }
}
