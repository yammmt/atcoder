// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u16,
    }

    if x < 600 {
        println!("8");
    } else if x < 800 {
        println!("7");
    } else if x < 1000 {
        println!("6");
    } else if x < 1200 {
        println!("5");
    } else if x < 1400 {
        println!("4");
    } else if x < 1600 {
        println!("3");
    } else if x < 1800 {
        println!("2");
    } else if x < 2000 {
        println!("1");
    }
}
