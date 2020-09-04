// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: u8,
        w: u8,
    }

    if s <= w {
        println!("unsafe");
    } else {
        println!("safe");
    }
}
