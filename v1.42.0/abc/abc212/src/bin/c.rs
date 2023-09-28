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
        an: [i64; n],
        mut bm: [i64; m],
    }
    // an.sort();
    bm.sort();

    let mut ans = std::i64::MAX;
    for a in &an {
        let lb = lower_bound(&bm, *a);
        if lb < m {
            ans = ans.min((*a - bm[lb]).abs());
        }
        if lb > 0 {
            ans = ans.min((*a - bm[lb - 1]).abs());
        }
    }

    println!("{}", ans);
}
