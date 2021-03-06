// :fu: :fu: 21-03 bitDP テンプレ部分で無限に死 こういうのほんと苦手

use proconio::input;
use std::collections::VecDeque;

const NOT_YET: usize = std::usize::MAX / 2;

fn shortest_paths(s: usize, n: usize, edges: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut ret = vec![NOT_YET; n];
    let mut vdq = VecDeque::new();
    ret[s] = 0;
    vdq.push_back((s, 0));
    while let Some(cur) = vdq.pop_front() {
        for v in &edges[cur.0] {
            if ret[*v] != NOT_YET {
                continue;
            }

            let cost = cur.1 + 1;
            ret[*v] = cost;
            vdq.push_back((*v, cost));
        }
    }
    ret
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvm: [(usize, usize); m],
        s: usize,
        k: usize,
        tk: [usize; k],
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }

    // 探索の始点となる s と tk 各点
    let mut new_tk = vec![s - 1];
    for t in &tk {
        new_tk.push(*t - 1);
    }

    let mut dists = vec![vec![NOT_YET; n]; k + 1];
    for (i, t) in new_tk.iter().enumerate() {
        dists[i] = shortest_paths(*t, n, &edges);
    }

    let bit_ptrn_max = (1 << new_tk.len()) as usize;
    // dp[訪問済][現在地]
    let mut dp = vec![vec![NOT_YET; new_tk.len()]; bit_ptrn_max];
    // 始点 0 (s - 1) で固定
    dp[0][0] = 0;
    for bit in 0..bit_ptrn_max {
        for i in 0..new_tk.len() {
            if dp[bit][i] == NOT_YET {
                continue;
            }

            // i 番目から移動して j 番目を訪問する
            for j in 0..new_tk.len() {
                let next_i = bit | (1 << j);
                dp[next_i][j] = dp[next_i][j].min(dp[bit][i] + dists[i][new_tk[j]]);
            }
        }
    }

    println!("{}", dp.last().unwrap().iter().min().unwrap());
}
