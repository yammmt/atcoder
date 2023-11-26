// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// use std::collections::BTreeSet;
// use std::collections::HashSet;
// use std::collections::HashMap;
use std::collections::VecDeque;

// static DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        sn: [Chars; n],
    }

    let mut p_start = (0, 0);
    let mut p_goal = (0, 0);
    for i in 0..n {
        for j in 0..n {
            if sn[i][j] == 'S' {
                p_start = (i, j);
            } else if sn[i][j] == 'G' {
                p_goal = (i, j);
            }
        }
    }

    // 制限 8s だが O(N^3) は無理そう

    // ぶつかるまで進む => 到達可能な範囲を一気に更新する, としたいが TLE
    // 回り込んで動ける場合がある？ない？
    // => ある, 下移動時に右 => 下 => 左で動ける
    // ~~ある k = kk で辿れるマスは kk-1 でも必ず辿れそう~~
    // 0-1BFS で求まる？
    // (今のマス, ここまでの移動数, k)
    // k でいけたなら k/2 でも倍の手数でいける
    // 1000 以下の素数は 168 個であり 1500*1500 に掛けると危険

    // k がある程度大きいと操作回数かなり減りそうであり枝刈りで間に合う？
    // k=n-1 のときは四隅にいなければ動けない

    let mut visited = vec![vec![false; n]; n];
    let mut que = VecDeque::new();
    // ((座標), 操作回数)
    que.push_back((p_start, 0));
}
