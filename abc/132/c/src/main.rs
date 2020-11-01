// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut dn: [u32; n],
    }
    dn.sort();
    if dn[n / 2] == dn[n / 2 - 1] { // unneeded
        println!("0");
    } else {
        println!("{}", dn[n / 2] - dn[n / 2 - 1]);
    }
}
