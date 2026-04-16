use proconio::input;
use std::cmp::Ordering;

fn cmp_frac(ab0: (isize, isize), ab1: (isize, isize)) -> Ordering {
    let c0 = ab0.0 / ab0.1;
    let c1 = ab1.0 / ab1.1;
    if c0 != c1 {
        c0.cmp(&c1)
    } else {
        cmp_frac((ab1.1, ab1.0 - c1 * ab1.1), (ab0.1, ab0.0 - c0 * ab0.1))
    }
}

fn main() {
    input! {
        n: usize,
        mut abn: [(isize, isize); n],
    }

    // 連分数展開
    abn.sort_unstable_by(|ab, cd| cmp_frac(*ab, *cd));

    abn.iter().for_each(|(a, b)| {
        println!("{a} {b}");
    });
}
