// TODO: ac_library にいれるだけらしい
// use ac_library::{Dsu, FenwickTree, SccGraph};

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

fn dfs_forward(edges: &Vec<Vec<usize>>, i: usize, visited: &mut Vec<bool>, order: &mut Vec<usize>) {
    visited[i] = true;

    for &v in &edges[i] {
        if visited[v] {
            continue;
        }

        dfs_forward(edges, v, visited, order);
    }

    order.push(i);
}

fn dfs_back(
    edges: &Vec<Vec<usize>>,
    i: usize,
    visited: &mut Vec<bool>,
    cn: &Vec<usize>,
    scores: &mut Vec<usize>,
) {
    visited[i] = true;

    for &v in &edges[i] {
        if visited[v] {
            continue;
        }

        dfs_back(edges, v, visited, cn, scores);
    }

    scores.push(cn[i]);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xn: [Usize1; n],
        cn: [usize; n],
    }

    // ループがあると誰かしらの不満度が発生する
    // 単純な円状のループであれば不満度最小の者を犠牲者とすればよい
    // ループとは強連結成分分解？
    // 実装だるいのでなにか賢いやりようがありそうだけれど

    let mut edge_forward = vec![vec![]; n];
    let mut edge_back = vec![vec![]; n];
    for (i, x) in xn.iter().enumerate() {
        edge_forward[i].push(*x);
        edge_back[*x].push(i);
    }

    let mut visited = vec![false; n];
    let mut order = vec![];
    for i in 0..n {
        if visited[i] {
            continue;
        }

        dfs_forward(&edge_forward, i, &mut visited, &mut order);
    }
    order.reverse();

    let mut ans = 0;
    let mut visited = vec![false; n];
    for i in order {
        if visited[i] {
            continue;
        }

        let mut scores = vec![];
        dfs_back(&edge_back, i, &mut visited, &cn, &mut scores);

        if scores.len() > 1 {
            ans += scores.iter().min().unwrap();
        }
    }

    println!("{ans}");
}
