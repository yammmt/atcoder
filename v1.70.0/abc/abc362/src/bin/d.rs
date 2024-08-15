use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        an: [usize; n],
        uvbm: [(Usize1, Usize1, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for (u, v, b) in uvbm {
        edges[u].push((v, b));
        edges[v].push((u, b));
    }
    let edges = edges;

    let mut ans = vec![DUMMY; n];
    let mut bh = BinaryHeap::new();
    // (重み, 頂点)
    // 重みに頂点はあってもなくてもよいはずだが, 持たせたほうが楽そう
    bh.push(Reverse((an[0], 0)));
    while let Some(Reverse((cost_cur, v_cur))) = bh.pop() {
        if ans[v_cur] != DUMMY {
            continue;
        }

        ans[v_cur] = cost_cur;

        for &(v_nxt, b_nxt) in &edges[v_cur] {
            if ans[v_nxt] != DUMMY {
                continue;
            }

            let cost_nxt = cost_cur + an[v_nxt] + b_nxt;
            bh.push(Reverse((cost_nxt, v_nxt)));
        }
    }

    for (i, a) in ans.iter().enumerate().skip(1) {
        print!("{a}");
        if i == n - 1 {
            println!();
        } else {
            print!(" ");
        }
    }
}
