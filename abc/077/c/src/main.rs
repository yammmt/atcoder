// -*- coding:utf-8-unix -*-

use proconio::input;

// v[i] > k を満たす最小の i を返す (or k 以下の v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn upper_bound(v: &[usize], k: usize) -> usize {
    let mut low: isize = -1;
    let mut high = v.len() as isize;

    while high - low > 1 {
        let mid = (low + high) / 2;
        if v[mid as usize] > k {
            high = mid;
        } else {
            low = mid;
        }
    }
    high as usize
}

// v[i] >= k を満たす最小の i を返す (or k より小さい v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn lower_bound(v: &[usize], k: usize) -> usize {
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
        mut a: [usize; n],
        b: [usize; n],
        mut c: [usize; n],
    }

    a.sort();
    c.sort();
    let mut ans = 0;
    for i in 0..n {
        let a_idx = lower_bound(&a[..], b[i]);
        let c_idx = upper_bound(&c[..], b[i]);

        // println!("a_idx: {}, b: {}, c_idx: {}", a_idx, b[i], c_idx);
        // println!("{}", a_idx * (n - c_idx));
        ans += a_idx * (n - c_idx);
    }
    println!("{}", ans);
}
