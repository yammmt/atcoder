// :fu: 22-02 インフレが激しい

use proconio::input;
use std::collections::BinaryHeap;

fn main() {
    input! {
        n: usize,
        m: usize,
        hn: [isize; n],
        uvm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for uv in &uvm {
        edges[uv.0 - 1].push(uv.1 - 1);
        edges[uv.1 - 1].push(uv.0 - 1);
    }
    let edges = edges;

    // 登る高さを最小化したいが Dijkstra は負辺を扱えない
    // ポテンシャルというらしい 知らな
}
