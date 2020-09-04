// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        w: u64,
        h: u64,
        x: u64,
        y: u64,
    }

    if w == 2 * x && h == 2 * y {
        println!("{} 1", (w * h) as f64 / 2.0);
    } else {
        println!("{} 0", (w * h) as f64 / 2.0);
    }
}
