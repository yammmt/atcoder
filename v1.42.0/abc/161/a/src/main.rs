// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        mut x: u8,
        mut y: u8,
        mut z: u8,
    }

    let mut tmp = x;
    x = y;
    y = tmp;

    tmp = x;
    x = z;
    z = tmp;

    println!("{} {} {}", x, y, z);
}
