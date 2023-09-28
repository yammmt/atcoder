// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u16,
    }
    if x == 0 {
        println!("1");
    } else {
        println!("0");
    }
}
