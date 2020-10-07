// -*- coding:utf-8-unix -*-

// TLE

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        p: [i64; n],
        c: [i64; n],
    }

    let mut ans: i64 = std::i64::MIN;
    for i in 0..n {
        let mut next_mas = p[i] - 1;
        let mut current_pts = 0;
        for _j in 0..k {
            current_pts += c[next_mas as usize];
            next_mas = p[next_mas as usize] - 1;
            if current_pts > ans {
                ans = current_pts;
            }
        }
    }
    println!("{}", ans);
}
