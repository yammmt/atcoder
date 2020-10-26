// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u16,
        s: String,
    }
    if a >= 3200 {
        println!("{}", s);
    } else {
        println!("red");
    }
}
