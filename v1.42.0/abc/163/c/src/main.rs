// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    let mut boss_cnt = vec![0; (n + 1) as usize];
    for _i in 0..n - 1 {
        input! {
            a: usize,
        }
        boss_cnt[a] += 1;
    }

    for i in 1..n + 1 {
        println!("{}", boss_cnt[i as usize]);
    }
}
