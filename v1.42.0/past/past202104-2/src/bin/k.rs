// use petgraph::unionfind::UnionFind;
use proconio::input;
// use proconio::marker::Chars;
// use std::collections::HashSet;
// use std::collections::HashMap;
// use std::collections::VecDeque;
// use permutohedron::heap_recursive;

fn main() {
    input! {
        n: usize,
        pun: [(i64, i64); n],
    }

    // 愚直は買う買わない全通りだが太陽系が消滅する

    // 点数を決め打ちして二分探索を考えると購入すべき商品の優先順がわかる?
    // もらえる R の数を決め打ちするには数字が大きすぎる気がする
    // U >= P の場合は必ず買うがそれ以外の場合はクーポン数に依存する
    // 端数に依存するので二分探索に言い換えられない…

    // 端数の都合で貪欲ではなさそう
    // Q 円払った歳の最高スコア (+ 端数) で DP しようにも 10^4 円のものを 10^5 個買う場合に TLE
    // 100 円につき 80 点未満の価値であれば買わないほうが良い?
}
