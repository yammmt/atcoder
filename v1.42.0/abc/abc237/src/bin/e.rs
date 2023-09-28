// :fu: 22-02 嘘解が通っていたらしいがどれほどかは知らない
// テストケースはしばらくあがっていない なんとかして
// https://qiita.com/u2dayo/items/3f1ea15ec3b71d021b43#e%E5%95%8F%E9%A1%8Cskiing

use proconio::input;
use std::collections::BinaryHeap;

const DUMMY: isize = std::isize::MIN / 4;

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
    // ポテンシャルというらしい 初遭遇？

    // 坂を下るぶんには "高さ + 楽しさ" は変化しない
    // 最終的な答えが "scores - 自身の高さ" であることから逆算すると良いのかも
    // heap に挿入する距離は自身以下の高さである頂点であればそのまま, 自身より高いならコストがその高さぶん悪化
    // これを BinaryHeap の挙動に合わせて反転して…
    // 始点の score を 0 にして高い方向に進む場合はそのぶん不方向に動かして最後に辻褄合わせた方が自然？
    let mut heap = BinaryHeap::new();
    let mut scores = vec![DUMMY; n];
    // (到達コスト, 頂点)
    heap.push((hn[0], 0));
    while let Some(cur) = heap.pop() {
        // println!("cur: {:?}", cur);
        // println!("  scores: {:?}", scores);
        // assert!(cur.0 <= 0);
        if scores[cur.1] != DUMMY {
            continue;
        }

        scores[cur.1] = cur.0;

        for v in &edges[cur.1] {
            if scores[*v] != DUMMY {
                continue;
            }

            heap.push(
                if hn[*v] <= hn[cur.1] {
                    (cur.0, *v)
                } else {
                    (cur.0 - hn[*v] + hn[cur.1], *v)
                }
            );
        }
    }
    // println!("{:?}", scores);

    // 最終的な到達点から (下りきれなかったぶんの) 高さを引いたものが答えになる
    let mut ans = DUMMY;
    for i in 0..n {
        ans = ans.max(scores[i] - hn[i]);
    }
    println!("{}", ans);
}
