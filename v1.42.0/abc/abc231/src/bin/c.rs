// 二分探索するだけだと diff 200 もいかないらしい

use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
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
        q: usize,
        mut an: [usize; n],
        xq: [usize; q],
    }
    an.sort_unstable();
    let an = an;

    for x in &xq {
        println!("{}", n - lower_bound(&an, *x));
    }
}
