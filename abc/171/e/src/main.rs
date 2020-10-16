// -*- coding:utf-8-unix -*-

// https://drken1215.hatenablog.com/entry/2020/06/22/122500

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let asum = a.iter().fold(0, |s, x| s ^ x);

    for i in 0..n {
        print!("{}", asum ^ a[i]);
        if i != n - 1 {
            print!(" ");
        }
    }
    println!("");
}
