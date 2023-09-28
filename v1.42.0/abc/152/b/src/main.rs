// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        a: u8,
        b: u8,
    }
    let mut astr = String::new();
    let mut bstr = String::new();
    for _ in 0..b {
        astr += &a.to_string();
    }
    for _ in 0..a {
        bstr += &b.to_string();
    }
    if astr < bstr {
        println!("{}", astr);
    } else {
        println!("{}", bstr);
    }
}
