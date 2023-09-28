// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        c: i64,
        bm: [i64; m],
        anm: [[i64; m]; n],
    }

    let mut ans = 0;
    for i in 0..n {
        let mut pts = c;
        for j in 0..m {
            pts += anm[i][j] * bm[j];
        }
        if pts > 0 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
