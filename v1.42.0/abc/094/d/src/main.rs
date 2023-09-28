// -*- coding:utf-8-unix -*-

use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn lower_bound(v: &[i64], k: i64) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] >= k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    // n <= 10^5 だし全部舐めたほうが確実
    a.sort();
    let a1 = a[n - 1];
    let mut a2_idx = lower_bound(&a[..n - 1], (a1 / 2) as i64);
    if a2_idx == n - 1 {
        // not found
        println!("{} {}", a1, a[0]);
        return;
    }

    // println!("a2_idx: {}, a2: {}", a2_idx, a[a2_idx]);
    if a2_idx > 0 {
        if (a[a2_idx - 1] - a1 / 2).abs() < (a[a2_idx] - a1 / 2).abs() {
            a2_idx -= 1;
        }
    }

    println!("{} {}", a1, a[a2_idx]);
}
