// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
        b: [u64; m],
    }

    let mut ans = 0;
    let mut total_time: u64 = b.iter().sum();
    let mut b_idx = m;

    for a_idx in 0..a.len() + 1 {
        while b_idx > 0 && total_time > k {
            b_idx -= 1;
            total_time -= b[b_idx];
        }
        if total_time > k {
            break;
        }

        ans = ans.max(a_idx + b_idx);
        if a_idx == n {
            break;
        }

        total_time += a[a_idx];
    }
    println!("{}", ans);
}
