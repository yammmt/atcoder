// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: [i64; n],
    }

    let mut ans = std::i64::MAX;
    for i in 0..n - k + 1 {
        // println!("edge: {} {}", x[i], x[i+k-1]);
        let dist = if x[i] * x[i + k - 1] < 0 {
            // 異符号なら二点間の距離に小さい方から戻ってくるための小さい方の絶対値足す
            // もしくは絶対値の小さい方に行く -> 原点に戻る -> 絶対値の大きい方に行く
            // (x[i] - x[i + k - 1]).abs() + x[i].abs().min(x[i + k - 1].abs())
            x[i].abs() + x[i + k - 1].abs() + x[i].abs().min(x[i + k - 1].abs())
        } else {
            // 同符号なら遠い方に行くだけ
            x[i].abs().max(x[i + k - 1].abs())
        };
        // println!("{}", dist);
        ans = ans.min(dist);
    }
    println!("{}", ans);
}
