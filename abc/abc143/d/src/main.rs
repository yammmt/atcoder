// -*- coding:utf-8-unix -*-

use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k より小さい v の要素数)
// 見つからなければ `v.len()` を返す
fn lower_bound(v: &[u32], k: u32) -> usize {
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
        mut l: [u32; n],
    }

    l.sort();
    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let k = lower_bound(&l[j + 1..], l[i] + l[j]);
            ans += k;
        }
    }
    println!("{}", ans);
}
