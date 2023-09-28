// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
    }
    let mut ans = 0;
    for i in &ab {
        ans += (i.0 + i.1) * (i.1 - i.0 + 1) / 2;
    }
    println!("{}", ans);
}
