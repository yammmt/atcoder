// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [u32; n],
    }

    a.sort();
    a.reverse();
    let mut a_point: u32 = 0;
    let mut b_point: u32 = 0;
    for i in 0..a.len() {
        match i % 2 {
            0 => a_point += a[i],
            1 => b_point += a[i],
            _ => {},
        }
    }
    println!("{}", a_point - b_point);
}
