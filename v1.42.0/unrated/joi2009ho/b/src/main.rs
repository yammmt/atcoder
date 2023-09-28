// -*- coding:utf-8-unix -*-

use proconio::input;

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
        d: usize,
        n: usize,
        m: usize,
        mut s: [usize; n - 1],
        k: [usize; m],
    }
    s.push(0);
    s.sort();

    let mut ans = 0;
    for i in 0..m {
        let lb = lower_bound(&s[..], k[i]);
        if lb == 0 {
            ans += k[i];
        } else if lb == s.len() {
            ans += (k[i] - s[s.len() - 1]).min(d - k[i]);
        } else {
            ans += (s[lb] - k[i]).min(k[i] - s[lb - 1]);
        }
    }
    println!("{}", ans);
}
