// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if s.contains("A") && s.contains("B") {
        println!("Yes");
    } else {
        println!("No");
    }
}
