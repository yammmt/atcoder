// use ac_library::SccGraph;
// use itertools::Itertools;
// use permutohedron::heap_recursive;
// use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        abn: [(Usize1, Usize1); n - 1],
        uvcq: [(Usize1, Usize1, usize)],
    }

    // クエリを逆順にして愚直に色がなければ更新, とすると TLE しそう
    // 最短を求める部分で 10^5 回の BFS をしてしまうので
    // この点数だと最短経路を先に求めておく手法も使えなさそう
    // LCA?
    // DFS で片方の頂点が入ってからもう片方の頂点が出るまでの間に
    // 探索が終わらなかった点は最短経路の通過点ではある
    // ordered set で対象点を絞ってとかできそうだが

    // TODO: LCA っぽい, 出直す
}
