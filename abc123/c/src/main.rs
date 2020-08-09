// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    }

    let min = a.min(b.min(c.min(d.min(e))));
    println!("{}", (n  + min - 1) / min + 4);
}
