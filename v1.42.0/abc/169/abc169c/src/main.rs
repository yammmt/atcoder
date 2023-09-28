// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u64,
        b: String,
    }

    let v: Vec<_> = b.split('.').collect();
    let b_int: u64 = v[0].parse().unwrap();
    let b_dec: u64 = v[1].parse().unwrap();

    let mut ans = a * b_int + (a * b_dec) / 100;
    println!("{}", ans);
}
