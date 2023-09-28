// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        x: i32,
    }

    println!(
        "{}",
        if x >= a && x <= a + b {
            "YES"
        } else {
            "NO"
        }
    );
}
