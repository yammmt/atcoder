// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
    }

    for x in -100..101 {
        for y in -100..101 {
            if x + y == a && x - y == b {
                println!("{} {}", x, y);
                return;
            }
        }
    }
}
