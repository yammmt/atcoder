// 53min 1WA (29min): 方針間違えて TLE

use proconio::input;
use std::collections::BinaryHeap;

const DUMMY: isize = std::isize::MAX / 3;

fn shortest_pathes(n: usize, edges: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut ret = vec![DUMMY; n];

    let mut bh = BinaryHeap::new();
    // 経過時間, 現在地
    bh.push((0, 0));
    while let Some(cur) = bh.pop() {
        if ret[cur.1] != DUMMY {
            continue;
        }

        ret[cur.1] = -cur.0;
        for e in &edges[cur.1] {
            if ret[e.0] == DUMMY {
                bh.push((cur.0 - e.1 as isize, e.0));
            }
        }
    }

    ret.iter().map(|&r| r as usize).collect::<Vec<usize>>()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        t: usize,
        an: [usize; n],
        abcm: [(usize, usize, usize); m],
    }
    let mut edges = vec![vec![]; n];
    let mut redges = vec![vec![]; n];
    for abc in &abcm {
        edges[abc.0 - 1].push((abc.1 - 1, abc.2));
        redges[abc.1 - 1].push((abc.0 - 1, abc.2));
    }

    // 強連結成分に近い
    // 1 基準でそれぞれのマスにいくまでの最短時間, マスから戻ってくる最短時間をすべて求める
    // マス n に行って戻ってくるための時間を出し, マス n にできるだけ滞在する
    let from_one = shortest_pathes(n, &edges);
    let to_one = shortest_pathes(n, &redges);
    let mut ans = an[0] * t;
    for i in 0..n {
        if from_one[i] + to_one[i] >= t {
            continue;
        }

        ans = ans.max(an[i] * (t - from_one[i] - to_one[i]));
    }

    println!("{}", ans);
}
