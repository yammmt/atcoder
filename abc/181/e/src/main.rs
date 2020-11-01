// -*- coding:utf-8-unix -*-

use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
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
        m: usize,
        mut hn: [i64; n],
        mut wm: [i64; m],
    }
    hn.sort();
    wm.sort();

    // 間はどこに入っても同じなので適当に入れる
    let mid = 0;

    // 先頭に入る
    let mut chibi = std::u64::MAX;
    if wn[0] <= hn[0] {
        let teacher_idx = lower_bound(&wm[i], hn[0]);
        let teacher = wm[teacher_idx];
        chibi = (hn[0] - teacher);
        let mut idx_small = 1;
        while idx_small < n - 1 {
            chibi += hn[idx_small + 1] - hn[idx_small];
            idx_small += 2;
        }
    }

    // 末尾に入る
    let kyodai = std::u64::MAX;
    if wm[m - 1] >= hn[n - 1] {
    }

    // 三通りの最小値が答え
    let ans = mid.min(chibi.min(kyodai));
    println!("{}", ans);
}
