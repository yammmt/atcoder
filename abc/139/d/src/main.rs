// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    println!("{}", n*(n-1)/2);
}
