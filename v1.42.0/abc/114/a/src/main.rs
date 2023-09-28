// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u8,
    }
    println!(
        "{}",
        match x {
            3 | 5 | 7 => "YES",
            _ => "NO",
        }
    )
}
