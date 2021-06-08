// 20min? 1WA
// WA: 同じ家の組

// 撤退された問題という雰囲気ではない

use proconio::input;

fn make_tuple(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    input! {
        n: usize,
        _xyn: [(i64, i64); n],
    }

    let mut xyn = vec![];
    for (i, xy) in _xyn.iter().enumerate() {
        xyn.push((xy.0, xy.1, i));
    }

    let mut candidates = vec![];

    xyn.sort_unstable();
    candidates.push(((xyn[0].0 - xyn[n - 1].0).abs(), make_tuple(xyn[0].2, xyn[n - 1].2)));
    if n == 3 {
        candidates.push(((xyn[0].0 - xyn[1].0).abs(), make_tuple(xyn[0].2, xyn[1].2)));
        candidates.push(((xyn[1].0 - xyn[2].0).abs(), make_tuple(xyn[1].2, xyn[2].2)));
    }
    if n > 3 {
        candidates.push(((xyn[0].0 - xyn[n - 2].0).abs(), make_tuple(xyn[0].2, xyn[n - 2].2)));
        candidates.push(((xyn[1].0 - xyn[n - 1].0).abs(), make_tuple(xyn[1].2, xyn[n - 1].2)));
    }

    xyn.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    candidates.push(((xyn[0].1 - xyn[n - 1].1).abs(), make_tuple(xyn[0].2, xyn[n - 1].2)));
    if n == 3 {
        candidates.push(((xyn[0].1 - xyn[1].1).abs(), make_tuple(xyn[0].2, xyn[1].2)));
        candidates.push(((xyn[1].1 - xyn[2].1).abs(), make_tuple(xyn[1].2, xyn[2].2)));
    }
    if n > 3 {
        candidates.push(((xyn[0].1 - xyn[n - 2].1).abs(), make_tuple(xyn[0].2, xyn[n - 2].2)));
        candidates.push(((xyn[1].1 - xyn[n - 1].1).abs(), make_tuple(xyn[1].2, xyn[n - 1].2)));
    }

    candidates.sort_unstable();
    candidates.reverse();
    for c in candidates.iter().skip(1) {
        if c.1 != candidates[0].1 {
            println!("{}", c.0);
            return;
        }
    }
}
