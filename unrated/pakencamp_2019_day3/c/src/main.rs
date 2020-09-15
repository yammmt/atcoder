// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[u64; m]; n],
    }

    let mut ans = 0u64;
    for i in 0..m {
        for j in i + 1..m {
            let mut current = 0u64;
            for k in 0..n {
                current += a[k][i].max(a[k][j]);
            }
            ans = ans.max(current);
        }
    }
    println!("{}", ans);
}
