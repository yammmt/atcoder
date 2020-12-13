// -*- coding:utf-8-unix -*-

// :fu:

use proconio::input;

fn xorsum(n: u64) -> u64 {
    let one_num = (n + 1) / 2;
    match n % 2 {
        0 => (one_num % 2) ^ n,
        _ => one_num % 2,
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    println!(
        "{}",
        match a {
            0 => xorsum(b),
            _ => xorsum(b) ^ xorsum(a - 1),
        }
    );
}
