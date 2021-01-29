// 6.5min

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
        wn: [i64; n],
    }

    let mut vans = vec![wn[0]];
    for w in &wn {
        let pos = lower_bound(&vans, *w);
        if pos == vans.len() {
            vans.push(*w);
        } else {
            vans[pos] = *w;
        }
    }

    println!("{}", vans.len());
}
