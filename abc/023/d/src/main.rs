// -*- coding:utf-8-unix -*-

// thanks to: https://algo-logic.info/abc023d/

use proconio::input;

fn main() {
    input! {
        n: usize,
        balloon: [(usize, usize); n],
    }

    let mut ng = 0;
    let mut ok = std::usize::MAX / 2;
    while ok - ng > 1 {
        let mid = (ok + ng) / 2;
        if balloon.iter().any(|b| b.0 > mid) {
            ng = mid;
            // println!("[pruning] ng: {}", ng);
            continue;
        }

        let mut vcount = vec![0; n];
        for i in 0..n {
            // 風船を割るまでの制限時間
            vcount[((mid - balloon[i].0) / balloon[i].1).min(n - 1)] += 1;
        }
        // println!("{:?}", vcount);

        if vcount[0] > 1 {
            ng = mid;
            continue;
        }
        for i in 1..n {
            // 時刻 i までに割られるべき風船の数
            vcount[i] += vcount[i - 1];
            if vcount[i] > i + 1 {
                ng = mid;
                break;
            }
        }

        if ng != mid {
            ok = mid;
            // println!("ok = {}", ok);
        }
    }
    println!("{}", ok);
}
