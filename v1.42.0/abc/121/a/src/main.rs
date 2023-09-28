// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        hh: usize,
        ww: usize,
    }
    println!(
        "{}",
        (h - hh) * (w - ww)
    );
}
