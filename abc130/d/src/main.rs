// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let mut sum = 0;
    let mut ans = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && sum < k {
            sum += a[right];
            right += 1;
        }
        if sum < k {
            break;
        }

        ans += (n - right + 1) as u64;

        if right == left {
            right += 1;
        } else {
            sum -= a[left];
        }
    }
    println!("{}", ans);
}
