use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x3: [usize; 3],
        s: Chars,
        abcm: [(Usize1, Usize1, usize); m],
    }

    // (vertex, cost)
    let mut edges = vec![vec![]; n];
    for (a, b, c) in abcm {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    // ワープを毎度 heap にいれると TLE する
    // ワープを使うなら, 起点はワープ元最短コスト点に限定してしまってよいので枝刈りができる

    let mut costs = vec![DUMMY; n];
    let mut heap = BinaryHeap::new();
    let mut warp_inserted = [false; 3];
    // (cost, v)
    heap.push((Reverse(0), 0));
    while let Some((Reverse(cost_cur), v_cur)) = heap.pop() {
        if costs[v_cur] != DUMMY {
            continue;
        }

        costs[v_cur] = cost_cur;

        for &(v_nxt, cost_added) in &edges[v_cur] {
            // 枝刈りせずともよいが
            if costs[v_nxt] != DUMMY {
                continue;
            }

            heap.push((Reverse(cost_cur + cost_added), v_nxt));
        }

        // warp
        let warp_type_cur = (s[v_cur] as u8 - b'A') as usize;
        if warp_inserted[warp_type_cur] {
            continue;
        }

        for i in 0..n {
            let warp_type_nxt = (s[i] as u8 - b'A') as usize;
            if warp_type_nxt == warp_type_cur {
                continue;
            }

            // 枝刈りしてもよいが
            // ちゃんと分岐書くと辛いので [0, 1, 2] の和から復元する
            // 0 -> 0 などのパスを先に潰しているので, これで通る
            let cost_added = x3[(warp_type_cur + warp_type_nxt - 1) % 3];
            heap.push((Reverse(cost_cur + cost_added), i));
        }
        warp_inserted[warp_type_cur] = true;
    }
    // println!("{:?}", costs);

    println!("{}", costs[n - 1]);
}
