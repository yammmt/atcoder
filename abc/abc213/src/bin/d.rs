// :fu: :fu: 21-08 数問でないが苦手 制約を読もう

use proconio::input;
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        n: usize,
        abn1: [(usize, usize); n - 1],
    }
    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }
    edges.iter_mut().for_each(|e| e.sort_unstable());
    edges.iter_mut().for_each(|e| e.reverse());
    let edges = edges;

    let mut visited = vec![false; n];
    let mut zero_not_visited = HashSet::new();
    edges[0].iter().for_each(|&v| {
        zero_not_visited.insert(v);
    });
    let mut ans = vec![];
    let mut vdq = VecDeque::new();
    // 現在地
    vdq.push_back(0);
    while let Some(cur) = vdq.pop_front() {
        ans.push(cur + 1);
        // if visited[cur] {
        //     continue;
        // }
        if cur == 0 && zero_not_visited.is_empty() {
            break;
        } else if visited[cur] {
            continue;
        }

        visited[cur] = true;
        if zero_not_visited.contains(&cur) {
            zero_not_visited.remove(&cur);
        }

        // 番号が最も小さい都市
        for v in &edges[cur] {
            if !visited[*v] {
                // 直前に訪れた都市
                vdq.push_front(cur);
                vdq.push_front(*v);
            }
        }
        // println!("vdq: {:?}", vdq);
    }
    // println!("ans: {:?}", ans);

    for (i, a) in ans.iter().enumerate() {
        print!("{}", a);
        if i == ans.len() - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
