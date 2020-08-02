// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
        a: [u64; n],
    }

    for i in (k + 1)..(n + 1) {
        if a[(i - k - 1) as usize] >= a[(i - 1) as usize] {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
