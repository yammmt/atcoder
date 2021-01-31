// -*- coding:utf-8-unix -*-

// 数問

use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }
    let one_time = (n - m) * 100 + 1900 * m;
    let pass = 2i32.pow(m as u32);
    println!("{}", one_time * pass);
}
