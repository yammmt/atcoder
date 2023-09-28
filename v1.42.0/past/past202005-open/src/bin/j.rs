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
        am: [u32; m],
    }

    // in asc order
    let mut childlen = vec![0; n];
    for a in &am {
        if childlen[0] >= *a {
            println!("-1");
            continue;
        }

        let eater = lower_bound(&childlen, *a);
        childlen[eater - 1] = *a;
        println!("{}", n - eater + 1);
    }
}
