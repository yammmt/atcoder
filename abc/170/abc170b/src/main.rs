// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        x: u16,
        y: u16,
    }

    for i in 0..x + 1 {
        if 2 * i + 4 * (x - i) == y {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
