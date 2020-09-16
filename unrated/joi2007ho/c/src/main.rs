// -*- coding:utf-8-unix -*-

// 参考: https://qiita.com/rudorufu1981/items/071aaf6eaccdce2e4295
// > 正方形を作りたければ、一つのベクトルに対して x と y を入れ替えて片方に - を付ける

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        p: [(i64, i64); n],
    }

    // HashSet 使わずに [5000][5000] のベクトルを用意すると stack overflow した
    let mut hs = HashSet::new();
    for i in &p {
        hs.insert(i);
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i + 1..n {
            let area = (p[i].0 - p[j].0).pow(2) + (p[i].1 - p[j].1).pow(2);
            if area <= ans {
                continue;
            }

            // 判定は片方向だけで良い
            // 全通り試すのでどこかで引っ掛かるし、判定回数減る分若干高速になる
            let vx = p[j].0 - p[i].0;
            let vy = p[j].1 - p[i].1;
            let x3 = p[i].0 - vy;
            let y3 = p[i].1 + vx;
            let x4 = p[j].0 - vy;
            let y4 = p[j].1 + vx;
            if hs.contains(&(x3, y3)) && hs.contains(&(x4, y4)) {
                ans = area;
            }
        }
    }
    println!("{}", ans);
}
