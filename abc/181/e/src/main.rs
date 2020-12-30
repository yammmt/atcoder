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
        m: usize,
        mut hn: [i64; n],
        wm: [i64; m],
    }
    hn.sort_unstable();

    let mut from_front = vec![0; n + 1];
    for i in 1..n + 1 {
        from_front[i] = match i % 2 {
            0 => from_front[i - 1] + hn[i - 1] - hn[i - 2],
            _ => from_front[i - 1],
        };
    }
    // println!("{:?}", from_front);

    let mut from_rear = vec![0; n + 1];
    for i in (0..n).rev() {
        from_rear[i] = match i % 2 {
            0 => from_rear[i + 1],
            _ => from_rear[i + 1] + hn[i + 1] - hn[i],
        };
    }
    // println!("{:?}", from_rear);

    let mut ans = std::i64::MAX;
    for w in &wm {
        let pos = lower_bound(&hn, *w);
        let mut cur = from_front[pos] + from_rear[pos];
        cur += match pos % 2 {
            0 => hn[pos] - *w,
            _ => *w - hn[pos - 1],
        };
        ans = ans.min(cur);
    }

    println!("{}", ans);
}
