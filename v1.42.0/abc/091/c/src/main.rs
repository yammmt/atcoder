// -*- coding:utf-8-unix -*-

// :fu: 二部グラフの最大マッチング問題
// https://www.youtube.com/watch?v=DqqPuIZvJTk

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        mut abn: [(usize, usize); n],
        mut cdn: [(usize, usize); n],
    }
    abn.sort();
    cdn.sort();
    // println!("abn: {:?}", abn);
    // println!("cdm: {:?}", cdn);

    // 点数の制約は緩い
    let mut used = HashSet::new();
    for cd in &cdn {
        let mut candidate = (n + 1, -1);
        for (i, ab) in abn.iter().enumerate() {
            if used.contains(&i) {
                continue;
            }

            // x 座標条件
            if ab.0 >= cd.0 {
                break;
            }

            // y 座標最大のものに更新する
            if ab.1 < cd.1 && ab.1 as isize > candidate.1 {
                candidate = (i, ab.1 as isize);
            }
        }

        if candidate.0 != n + 1 {
            used.insert(candidate.0 as usize);
        }
    }

    println!("{}", used.len());
}
