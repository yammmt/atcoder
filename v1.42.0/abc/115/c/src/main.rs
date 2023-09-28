// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: u32,
        k: u32,
        mut h: [u64; n],
    }

    h.sort();
    let mut ans = std::u64::MAX;
    let mut i = 0;
    while i + k - 1 < n {
        if h[(i + k - 1) as usize] - h[i as usize] < ans {
            ans = h[(i + k - 1) as usize] - h[i as usize];
        }
        i += 1;
    }
    println!("{}", ans);
}
