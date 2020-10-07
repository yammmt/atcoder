// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        d: u32,
        t: u32,
        s: u32,
    }
    if s * t >= d {
        println!("Yes");
    } else {
        println!("No");
    }
}
