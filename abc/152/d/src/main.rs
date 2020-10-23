// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut vcmb = vec![vec![0; 10]; 10];
    let mut vup = 1;
    for i in 1..n + 1 {
        if i == 10 * vup {
            vup *= 10;
        }
        if i < 10 {
            vcmb[i as usize][i as usize] += 1;
        } else {
            vcmb[(i / vup) as usize][(i % 10) as usize] += 1;
        }
    }

    let mut ans = 0;
    for i in 1..10 {
        for j in 1..10 {
            ans += vcmb[i][j] * vcmb[j][i];
        }
    }
    println!("{}", ans);
}
