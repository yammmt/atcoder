// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: Vec<char>,
    }

    if s[2] == s [3] && s[4] == s[5] {
        println!("Yes");
    } else {
        println!("No");
    }
}
