// -*- coding:utf-8-unix -*-

use proconio::input;
use proconio::marker::Chars;

// DOES NOT WORK

fn main() {
    input! {
        s: String,
    }

    let v: Vec<char> = s.chars().collect();
    println!("{}", v.len());
}
