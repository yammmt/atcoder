// 実装が脳筋

use proconio::input;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        uvn: [(usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvn {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }
    let edges = edges;

    // 答えを n 行出力するので始点を固定してで愚直に加算すると TLE
    // 適当に頂点 1 から BFS すれば一行目の答えは得られる
    // 頂点を一つ移動するとパスを区切って全頂点が近くなるか遠くなるかする
    // 自身の子要素の数を前計算できれば十分だが

    // 木 DP だっけ？
    let mut children_num = vec![0i64; n];
    let mut comes_from = vec![None; n];
    let mut vdq = VecDeque::new();
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_back() {
        // println!("cur: {}", cur);
        // println!("  {:?}", vdq);
        if children_num[cur] == 0 {
            // 初回は子をキューに突っ込む
            vdq.push_back(cur);
            children_num[cur] = 1;
            for v in &edges[cur] {
                if children_num[*v] == 0 {
                    comes_from[*v] = Some(cur);
                    vdq.push_back(*v);
                }
            }
        } else {
            // 二度目の突入は親に子の数を伝えるだけ
            if let Some(parent) = comes_from[cur] {
                children_num[parent] += children_num[cur];
            }
        }
    }
    // println!("{:?}", children_num);

    // 一度 BFS で根から全長点への距離を求める
    let mut ans_0 = 0i64;
    let mut visited = vec![false; n];
    let mut vdq = VecDeque::new();
    vdq.push_back((0, 0));
    visited[0] = true;
    while let Some(cur) = vdq.pop_front() {
        ans_0 += cur.1;
        for v in &edges[cur.0] {
            if !visited[*v] {
                vdq.push_back((*v, cur.1 + 1));
                visited[*v] = true;
            }
        }
    }

    // 以降は子の数を見て値を更新させる
    let mut ans = vec![0; n];
    let mut visited = vec![false; n];
    let mut vdq = VecDeque::new();
    ans[0] = ans_0;
    visited[0] = true;
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_back() {
        for v in &edges[cur] {
            if !visited[*v] {
                ans[*v] = ans[cur] - children_num[*v] + (n as i64 - children_num[*v]);
                vdq.push_back(*v);
                visited[*v] = true;
            }
        }
    }

    for a in &ans {
        println!("{}", a);
    }
}
