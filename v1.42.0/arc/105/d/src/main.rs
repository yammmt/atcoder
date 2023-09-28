// -*- coding:utf-8-unix -*-

// NOT WORK

use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            a: [u64; n],
        }
        if n % 2 == 0 {
            println!("First");
        } else {
            println!("Second");
        }
    }
}
