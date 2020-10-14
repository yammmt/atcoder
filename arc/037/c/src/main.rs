// -*- coding:utf-8-unix -*-

// http://mmxsrup.hatenablog.com/entry/2016/10/23/141536
// https://phyllo-algo.hatenadiary.org/entry/20150419/1429410543

use proconio::input;

// v[i] > k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn upper_bound(v: &[u64], k: u64) -> usize {
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

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
        mut b: [u64; n],
    }
    a.sort();
    b.sort();

    // left: この数以下の要素は k 個未満
    // right: この数以下の要素は k 個以上
    // left-right 切り替わる地点は明らかに a[i] * b[j] の積で表すことができる値となる.
    // そうでなければ数値比較する要素数が変化することはない.
    // `let mut left = a[0] * b[0];` としてはならない. k = 1 時に探索開始位置がバグる.
    // 安全策で殴るなら [0, std::u64::MAX] で開始してやっても余裕を持って間に合う (言語差は知らない).
    let mut left = a[0] * b[0] - 1;
    let mut right = a[n - 1] * b[n - 1];
    while right - left > 1 {
        // println!("{} {}", left, right);
        let mid = (right + left) / 2;
        // println!("mid: {}", mid);
        let mut under_k_num = 0;
        for i in &a {
            under_k_num += upper_bound(&b[..], mid / *i);
        }
        if under_k_num >= k {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
