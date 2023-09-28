// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    }

    if a >= 0 && c >= 0 {
        println!("{}", b * d);
    } else if b <= 0 && d <= 0 {
        println!("{}", a * c);
    } else {
        println!("{}", (a * c).max((a * d).max((b * c).max(b * d))));
    }
}
