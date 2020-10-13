// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    for i in &a {
        if *i % 2 == 0 &&  (*i % 3 != 0 && *i % 5 != 0) {
            println!("DENIED");
            return;
        }
    }
    println!("APPROVED");
}
