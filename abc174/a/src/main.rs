// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: i32,
    }

    if x < 30 {
        println!("No");
    } else {
        println!("Yes");
    }
}
