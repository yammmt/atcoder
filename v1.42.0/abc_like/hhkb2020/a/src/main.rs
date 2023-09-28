// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    if s == "Y" {
        if t == "a" {
            println!("A");
        } else if t == "b" {
            println!("B");
        } else {
            println!("C");
        }
    } else {
        println!("{}", t);
    }
}
