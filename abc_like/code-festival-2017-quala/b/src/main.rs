// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
        k: i32,
    }

    for i in 0..n + 1 {
        for j in 0..m + 1 {
            if (m * i) + j * (n - 2 * i) == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
