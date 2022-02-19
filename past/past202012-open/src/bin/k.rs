use proconio::input;
use proconio::marker::Chars;
use std::collections::{BinaryHeap, HashSet};

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

impl<T: std::convert::Into<f64>> std::convert::From<f64> for MyEq<T> {
    fn from(from: f64) -> Self {
        from.into()
    }
}

fn main() {
    input! {
        s44: [Chars; 4],
    }
    let dir = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 盤面の状態は 2^16 個と少ないので全探索が間に合いそう
    // BinaryHeap の方が重複を消せて良いが f64 に対して使いたくない…

    // FIXME: "どちらかに当たれば良い" という遷移が表現できていない
    let mut ans: f64 = std::f64::MAX;
    let mut heap = BinaryHeap::new();
    // 今の期待値, 期待値, 盤面
    heap.push((MyEq(1.0), s44));
    let mut hs = HashSet::new();
    while let Some(cur) = heap.pop() {
        println!("{:?}", cur.0);
        for line in &cur.1 {
            println!("{:?}", line);
        }
        if hs.contains(&cur.1) {
            println!("  skip");
            continue;
        }

        hs.insert(cur.1.clone());
        // 投げ先を全探索
        for i in 0..4 {
            for j in 0..4 {
                // (i, j) に投げると周辺に 1/5 であたる
                for d in &dir {
                    let hit_i_i = i as isize + d.0;
                    let hit_j_i = j as isize + d.1;
                    if hit_i_i < 0 || hit_i_i > 3 || hit_j_i < 0 || hit_j_i > 3 {
                        continue;
                    }

                    let hit_i_u = hit_i_i as usize;
                    let hit_j_u = hit_j_i as usize;
                    if cur.1[hit_i_u][hit_j_u] == '.' {
                        // 盤面変わらず
                        continue;
                    }

                    let mut next_mato = cur.1.clone();
                    next_mato[hit_i_u][hit_j_u] = '.';

                    // 全マス踏み抜いた
                    let mut cleared = true;
                    'outer: for ii in 0..4 {
                        for jj in 0..4 {
                            if next_mato[ii][jj] == '#' {
                                cleared = false;
                                break 'outer;
                            }
                        }
                    }
                    let next_ex = 0.20 * f64::from(cur.0);
                    if cleared {
                        println!("  next_ex: {}", next_ex);
                        ans = ans.min(1.0 / next_ex);
                    } else {
                        heap.push((MyEq(next_ex), next_mato));
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
