use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        // 大きな塔
        xycn: [(isize, isize, usize); n],
        // 小さな塔
        xycm: [(isize, isize, usize); m],
    }

    // 何も考えず最小全域木をすると無駄に小さい塔を拾う可能性がある
    // 最小全域木に含まれる小さな塔の種類を全部試す？
    // 高々 2^5 通りでしかないから間に合う

    let dist_2 = |a: (isize, isize, usize), b: (isize, isize, usize)| {
        let cost_base = (a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1);
        if a.2 == b.2 {
            cost_base
        } else {
            cost_base * 100
        }
    };

    let mut ans = f64::MAX / 2.0;
    for bit in 0..2usize.pow(m as u32) {
        let mut is_connected = vec![false; m];
        let mut b = bit;
        for i in 0..m {
            if b % 2 == 1 {
                is_connected[i] = true;
            }
            b /= 2;
        }

        let mut uf = UnionFind::new(n + m);
        let mut heap = BinaryHeap::new();

        // 大きい塔と大きい塔
        for i in 0..n {
            for j in i + 1..n {
                heap.push(Reverse((dist_2(xycn[i], xycn[j]), i, j)));
            }
        }

        // 大きい塔と小さい塔
        for i in 0..n {
            for j in 0..m {
                if !is_connected[j] {
                    continue;
                }

                heap.push(Reverse((dist_2(xycn[i], xycm[j]), i, j + n)));
            }
        }

        // 小さい塔と小さい塔
        for i in 0..m {
            if !is_connected[i] {
                continue;
            }

            for j in i + 1..m {
                if !is_connected[j] {
                    continue;
                }

                heap.push(Reverse((dist_2(xycm[i], xycm[j]), i + n, j + n)));
            }
        }

        let mut cost = 0.0;
        while let Some(Reverse((cost_cur, i, j))) = heap.pop() {
            if uf.equiv(i, j) {
                continue;
            }

            uf.union(i, j);
            cost += (cost_cur as f64).sqrt();
            // println!("  {i} {j}, {cost}");
        }

        // println!("{:?}", uf);
        // println!("  {cost}");
        ans = ans.min(cost);
    }

    println!("{ans}");
}
