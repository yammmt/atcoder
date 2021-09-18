// 三点固定しようとすると重実装

use proconio::input;
use std::collections::HashSet;


fn main() {
    input! {
        n: usize,
        xyn: [(usize, usize); n],
    }

    // 2000C4 全探索は TLE
    // 一点決めると x/y 座標が同じ点を絞り出すだけ

    let mut all_pts = HashSet::new();
    xyn.iter().for_each(|xy| { all_pts.insert(xy); });

    let mut ans = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            let req0 = (xyn[i].0, xyn[j].1);
            let req1 = (xyn[j].0, xyn[i].1);
            let mut cur = vec![xyn[i], xyn[j], req0, req1];
            cur.sort_unstable();
            cur.dedup();
            if cur.len() != 4 {
                continue;
            }

            if all_pts.contains(&req0) && all_pts.contains(&req1) {
                ans.insert(cur);
            }
        }
    }
    // for a in &ans {
    //     println!("{:?}", a);
    // }

    println!("{}", ans.len());
}
