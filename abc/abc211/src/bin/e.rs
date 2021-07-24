// 400ms 弱であり嘘解かも この書き方ではエッジケースある癖にペナ率が低い

use proconio::input;
use proconio::marker::Chars;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        k: usize,
        snn: [Chars; n],
    }
    let dir = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    // 最大ケースは明らかにすべてが白マスの場合 (ex3)
    // 愚直全通り列挙すると 64C8 となり TLE
    // 左上を赤としてそこから右/下に拡張するといける？ならば 2^7 の bit 全探索
    //   だが回り込むように塗るパターンがあるのでこれでは不足 初手だけ右下の二択にする, ではだめ
    //   左端だけ固定すると重複が発生して TLE
    // 4^7 は大したことないので派手に枝刈りすれば間に合ったりしないか
    let mut ans = HashSet::new();
    let mut done = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if snn[i][j] == '#' {
                continue;
            }

            if k == 1 {
                ans.insert(vec![(i, j)]);
                continue;
            }

            let mut cur_search = HashSet::new();
            cur_search.insert(vec![(i, j)]);
            done.insert(vec![(i, j)]);
            for kk in 0..k - 1 {
                let mut next = HashSet::new();
                for vv in &cur_search {
                    for v in vv {
                        for d in &dir {
                            let next_i_i = v.0 as isize + d.0;
                            let next_j_i = v.1 as isize + d.1;
                            if next_i_i < 0
                                || next_j_i < 0
                                || next_i_i >= n as isize
                                || next_j_i >= n as isize
                            {
                                continue;
                            }

                            let next_i_u = next_i_i as usize;
                            let next_j_u = next_j_i as usize;
                            let mut next_vec = vv.clone();
                            next_vec.push((next_i_u, next_j_u));
                            next_vec.sort_unstable();
                            next_vec.dedup();
                            if snn[next_i_u][next_j_u] == '#'
                                || done.contains(&next_vec)
                                || next_vec.len() == vv.len()
                            {
                                continue;
                            }

                            done.insert(next_vec.clone());
                            next.insert(next_vec);
                        }
                    }
                }
                if kk == k - 2 {
                    for ne in &next {
                        ans.insert(ne.clone());
                    }
                } else {
                    cur_search = next;
                }
            }
        }
    }
    // println!("{:?}", ans);

    println!("{}", ans.len());
}
