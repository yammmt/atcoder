// -*- coding:utf-8-unix -*-

use proconio::input;

fn f(a: i64, b: i64, x: i64) -> i64 {
    (((a * x) as f64 / (b as f64)).floor() - a as f64 * (x as f64 / b as f64).floor()) as i64
}

fn main() {
    input! {
        a: i64,
        b: i64,
        n: i64,
    }

    let ans = if n >= b - 1 {
        f(a, b, b - 1)
    } else {
        f(a, b, n)
    };
    println!("{}", ans);
}
