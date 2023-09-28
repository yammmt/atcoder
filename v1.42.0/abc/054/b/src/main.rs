// -*- coding:utf-8-unix -*-

// 25min 1WA

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        m: usize,
        n: usize,
        a: [Chars; m],
        b: [Chars; n],
    }

    for h_base in 0..m - n + 1 {
        for w_base in 0..m - n + 1 {
            'outer: for i in 0..n {
                for j in 0..n {
                    if a[h_base + i][w_base + j] != b[i][j] {
                        break 'outer;
                    }

                    if i == n - 1 && j == n - 1 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
