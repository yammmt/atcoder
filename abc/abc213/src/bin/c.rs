use proconio::input;

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
        _h: usize,
        _w: usize,
        n: usize,
        abn: [(usize, usize); n],
    }

    let mut remained_cols = vec![];
    for ab in &abn {
        remained_cols.push(ab.0);
    }
    remained_cols.sort_unstable();
    remained_cols.dedup();

    let mut remained_rows = vec![];
    for ab in &abn {
        remained_rows.push(ab.1);
    }
    remained_rows.sort_unstable();
    remained_rows.dedup();

    for ab in &abn {
        println!(
            "{} {}",
            lower_bound(&remained_cols, ab.0) + 1,
            lower_bound(&remained_rows, ab.1) + 1
        );
    }
}
