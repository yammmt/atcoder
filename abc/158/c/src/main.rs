// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u16,
        b: u16,
    }

    for i in 1..1010 {
        if ((i as f64 * 0.08).floor() as u16 == a) && ((i as f64 * 0.10).floor() as u16 == b) {
        // if (i * 8 == a * 100) && (i == b * 10) {
            println!("{}", i);
            return;
        }
    }
    println!("-1");
}
