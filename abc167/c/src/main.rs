// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u8,
        m: u8,
        x: u32,
        v: [[u32; m + 1]; n]
    }

    let mut ans = std::u32::MAX;

    for bit in 0..2u16.pow(n as u32) {
        let mut score = Vec::with_capacity(m as usize);
        let mut cost = 0;
        for _i in 0..m {
            score.push(0);
        }
        for i in 0..n {
            if bit & (1 << i) > 0 {
                for j in 1..m + 1 {
                    score[j as usize - 1] += v[i as usize][j as usize]; // reverse?
                }
                cost += v[i as usize][0];
                if cost >= ans {
                    break;
                }
            }
            if score.iter().all(|&s| s >= x) {
                if cost < ans {
                    ans = cost;
                }
            }
        }
    }
    if ans == std::u32::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
