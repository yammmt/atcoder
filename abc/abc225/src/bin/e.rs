// デバッグ詰み

// 誤差が重いが editorial は誤差への言及が見えないのは慈悲がない (放送にはある)
// オペレータ定義でなく比較関数定義になる言語はないものか

use proconio::input;
use std::cmp::Ordering;

fn cmp_a(a: (i64, i64), b: (i64, i64)) -> std::cmp::Ordering {
    if a.0 == 0 && b.0 == 0 {
        Ordering::Equal
    } else if a.0 == 0 {
        Ordering::Greater
    } else if b.0 == 0 {
        Ordering::Less
    } else {
        (a.1 * b.0).cmp(&(b.1 * a.0))
    }
}

fn main() {
    input! {
        n: usize,
        mut xyn: [(i64, i64); n],
    }

    // 原点から (x_i - 1, y_i) および (x_i, y_i - 1) に対しての傾きを計算する
    // この傾きを重複部分を除きつつ並べられれば良い

    // f64 で傾きを求めると誤差でしぬ
    // 終了区間 (y/x-1) 昇順
    xyn.sort_unstable_by(|a, b| { cmp_a((a.0 - 1, a.1), (b.0 - 1, b.1)) });

    let mut ans = 0;
    let mut last = (0, -1);
    for xy in &xyn {
        if cmp_a((xy.0, xy.1 - 1), (last.0 - 1, last.1)) != Ordering::Less {
            last = *xy;
            ans += 1;
        }
    }

    println!("{}", ans);
}
