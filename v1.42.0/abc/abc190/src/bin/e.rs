// :fu: TSP (bitDP)
// データ構造の変換がだるい
// テストケース非公開

// bitDP 部分:
// https://atcoder.jp/contests/abc190/submissions/19904183

use proconio::input;
use std::collections::VecDeque;

const UNUSED: usize = std::usize::MAX / 2;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
        k: usize,
        ck: [usize; k],
    }
    let ck = ck.iter().map(|c| *c - 1).collect::<Vec<usize>>();

    let mut c2k = vec![UNUSED; n];
    for i in 0..k {
        c2k[ck[i]] = i;
    }
    // println!("{:?}", c2k);

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut dists = vec![vec![UNUSED; k]; k];
    for i in 0..k {
        dists[i][i] = 0;
    }
    for (i, c) in ck.iter().enumerate() {
        let mut vdq = VecDeque::new();
        let mut is_found = vec![false; n];
        vdq.push_back((*c, 1));
        is_found[*c] = true;
        while let Some(cur) = vdq.pop_front() {
            for e in &edges[cur.0] {
                if is_found[*e] {
                    continue;
                }

                if c2k[*e] != UNUSED {
                    dists[i][c2k[*e]] = cur.1;
                }
                vdq.push_back((*e, cur.1 + 1));
                is_found[*e] = true;
            }
        }
    }
    // println!("{:?}", dists);
    for i in 0..k {
        for j in 0..k {
            if dists[i][j] == UNUSED {
                println!("-1");
                return;
            }
        }
    }

    // dp[訪問済][現在地]
    let bit_row_max = (1 << k) as usize;
    let mut dp = vec![vec![UNUSED; k]; bit_row_max];
    for i in 0..k {
        dp[1 << i][i] = 0;
    }

    for bit in 1..bit_row_max {
        for i in 0..k {
            if dp[bit][i] == UNUSED {
                continue;
            }

            for j in 0..k {
                // 既に訪問済なら更新しない
                // if bit & (1 << j) > 0 {
                //     continue;
                // }

                let next_i = bit | (1 << j);
                dp[next_i][j] = dp[next_i][j].min(dp[bit][i] + dists[i][j]);
            }
        }
    }

    // +1: 始点分
    println!("{}", dp.last().unwrap().iter().min().unwrap() + 1);
}
