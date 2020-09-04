// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    if s.contains("RRR") {
        println!("3");
    } else if s.contains("RR") {
        println!("2");
    } else if s.contains("R") {
        println!("1");
    } else {
        println!("0");
    }
}
