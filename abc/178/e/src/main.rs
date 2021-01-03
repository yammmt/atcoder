// -*- coding:utf-8-unix -*-

// https://naoyat.hatenablog.jp/entry/k-dimension-manhattan-distance

use proconio::input;

fn main() {
    input! {
        n: usize,
        xyn: [(i64, i64); n],
    }

    let mut xmax = std::i64::MIN;
    let mut xmin = std::i64::MAX;
    let mut ymax = std::i64::MIN;
    let mut ymin = std::i64::MAX;
    for xy in &xyn {
        xmax = xmax.max(xy.0 - xy.1);
        xmin = xmin.min(xy.0 - xy.1);
        ymax = ymax.max(xy.0 + xy.1);
        ymin = ymin.min(xy.0 + xy.1);
    }

    println!(
        "{}",
        (xmax - xmin).max(ymax - ymin)
    );
}
