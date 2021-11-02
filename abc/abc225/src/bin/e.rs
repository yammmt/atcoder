// WA: 浮動小数点数誤差と幾何

// 誤差が重いが editorial は誤差への言及が見えない (放送にはある) (E 解く層には不要？)
// オペレータ定義でなく比較関数定義になる言語はないものか

use proconio::input;
use std::cmp::Ordering;

fn cmp_a(a: (i64, i64), b: (i64, i64)) -> std::cmp::Ordering {
    (a.1 * b.0).cmp(&(b.1 * a.0))
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
    // 傾き最小値の初期設定が (0, -1) では通らない. 実際に "フ" を書いてみればわかるが,
    // これでは y = x の線が引かれてしまうので傾き 1 未満の点に対して誤判定となる
    let mut last = (1, -1);
    for xy in &xyn {
        if cmp_a((xy.0, xy.1 - 1), (last.0 - 1, last.1)) != Ordering::Less {
            last = *xy;
            ans += 1;
        }
    }

    println!("{}", ans);
}
