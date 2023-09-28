// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        m: usize,
        star: [(i64, i64); m],
        n: usize,
        p: [(i64, i64); n],
    }

    let mut hs = HashSet::new();
    for i in &p {
        hs.insert(i);
    }

    // 移動後の星座の一点目
    for i in 0..n {
        // 移動前の星座の一点目
        for j in 0..m {
            let moved = (p[i].0 - star[j].0, p[i].1 - star[j].1);
            for k in 0..m {
                if !hs.contains(&(star[k].0 + moved.0, star[k].1 + moved.1)) {
                    break;
                }

                // 問題文より必ず解がある
                if k == m - 1 {
                    println!("{} {}", moved.0, moved.1);
                    return;
                }
            }
        }
    }
}
