// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        k: u8,
        s: String,
    }

    let v: Vec<char> = s.chars().collect();

    for i in 0..v.len().min(k as usize) {
        print!("{}", v[i as usize]);
    }

    if v.len() > k as usize {
        print!("...");
    }
    println!("");
}
