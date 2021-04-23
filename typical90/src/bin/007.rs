use proconio::input;

// v[i] >= k を満たす最小の i を返す (or k 以下である v の要素数)
// 見つからなければ `v.len()` を返す
// `v` はソートされていること
fn lower_bound(v: &[i32], k: i32) -> usize {
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
        mut an: [i32; n],
        q: usize,
        bq: [i32; q],
    }
    an.sort();
    for b in &bq {
        let lb = lower_bound(&an, *b);
        println!(
            "{}",
            if lb == 0 {
                an[0] - *b
            } else if lb == n {
                *b - an[n - 1]
            } else {
                (an[lb] - *b).min(*b - an[lb - 1])
            }
        );
    }
}
