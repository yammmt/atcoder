// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }
    for _ in 0..s.len() {
        print!("x");
    }
    println!("");
}
