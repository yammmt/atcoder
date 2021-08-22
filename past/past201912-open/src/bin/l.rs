// :fu: BinaryHeap に浮動小数点数を食わせるため Rust では文法でクソほど時間が取られる
// bit 全列挙 + プリム法 (最小全域木)

use proconio::input;
use std::collections::BinaryHeap;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct MyEq<T>(pub T);
impl<T: PartialEq> Eq for MyEq<T> {}
impl<T: PartialOrd> Ord for MyEq<T> {
    fn cmp(&self, other: &MyEq<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl<T: std::convert::Into<f64>> std::convert::From<MyEq<T>> for f64 {
    fn from(from: MyEq<T>) -> Self {
        from.0.into()
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xycn: [(f64, f64, usize); n],
        xycm: [(f64, f64, usize); m],
    }

    let mut ans = std::f64::MAX / 2.0;
    for i in 0..1 << m {
        // println!("i: {}", i);

        // 使う頂点集合を定める
        let mut vertexes = xycn.clone();
        for (j, xyc) in xycm.iter().enumerate() {
            if i >> j & 1 > 0 {
                vertexes.push(*xyc);
            }
        }
        let vertexes = vertexes;

        let mut score = 0.0;
        let mut bh = BinaryHeap::new();
        let mut visited = vec![false; vertexes.len()];
        // (コスト, 頂点番号)
        bh.push((MyEq(0.0), 0));
        while let Some(cur) = bh.pop() {
            // println!("cur: {:?}", cur);
            if visited[cur.1] {
                continue;
            }

            score += -1.0 * f64::from(cur.0);
            visited[cur.1] = true;
            if visited.iter().all(|&a| a) {
                break;
            }

            let x0 = vertexes[cur.1].0;
            let y0 = vertexes[cur.1].1;
            let c0 = vertexes[cur.1].2;
            for (j, v) in vertexes.iter().enumerate() {
                if j == cur.1 || visited[j] {
                    continue;
                }

                let x1 = v.0;
                let y1 = v.1;
                let c1 = v.2;
                let cost = if c0 == c1 {
                    ((x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1)).sqrt()
                } else {
                    10.0 * ((x0 - x1) * (x0 - x1) + (y0 - y1) * (y0 - y1)).sqrt()
                };
                bh.push((MyEq(-cost), j));
            }
        }

        ans = ans.min(score);
    }

    println!("{}", ans);
}
