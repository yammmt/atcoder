// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: [u32; 5],
    }

    for i in 0..x.len() {
        if x[i] == 0 {
            println!("{}", i + 1);
            break;
        }
    }
}
