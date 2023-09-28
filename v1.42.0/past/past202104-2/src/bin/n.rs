// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

// 愚直は N! 通りだが TLE
// 貪欲では体力消費が激しいものを考えると WA になりそう

// 減点幅を考えると後に選んだ a ないし先に選んだ b の影響度が高い
// 体力振り切ってもデメリットがないので b が重いものは最後に実行すると良さそう
// 活動数 N が小さく O(N^3) が間に合うが

// 三次元の DP?

fn main() {
    input! {
        n: usize,
        h: i64,
        abn: [(i64, i64); n],
    }
}
