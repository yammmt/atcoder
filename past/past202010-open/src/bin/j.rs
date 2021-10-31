use proconio::input;
use proconio::marker::Chars;
use std::collections::BinaryHeap;

const DUMMY: isize = std::isize::MAX / 2;

fn main() {
    input! {
        n: usize,
        m: usize,
        xab: isize,
        xac: isize,
        xbc: isize,
        s: Chars,
        abcm: [(usize, usize, isize); m],
    }

    let mut edges = vec![vec![]; n];
    for abc in &abcm {
        // (移動先, コスト)
        edges[abc.0 - 1].push((abc.1 - 1, abc.2));
        edges[abc.1 - 1].push((abc.0 - 1, abc.2));
    }
    let edges = edges;

    // ワープがなければ Dijkstra 典型
    // ワープを何度も処理してしまうと TLE するのでワープ格納判定を入れる
    let mut warp_from_a_inserted = false;
    let mut warp_from_b_inserted = false;
    let mut warp_from_c_inserted = false;

    let mut costs = vec![DUMMY; n];
    let mut bh = BinaryHeap::new();
    // (合計コスト, 移動先)
    bh.push((0, 0));
    while let Some(cur) = bh.pop() {
        // println!("{:?}", bh);
        if costs[cur.1] != DUMMY {
            continue;
        }

        costs[cur.1] = -cur.0;
        if cur.1 == n - 1 {
            break;
        }

        // 通常移動
        for v in &edges[cur.1] {
            if costs[v.0] != DUMMY {
                continue;
            }

            bh.push((cur.0 - v.1, v.0));
        }

        // ワープ
        if (s[cur.1] == 'A' && warp_from_a_inserted)
            || (s[cur.1] == 'B' && warp_from_b_inserted)
            || (s[cur.1] == 'C' && warp_from_c_inserted)
        {
            continue;
        }

        match s[cur.1] {
            'A' => {
                for (i, ss) in s.iter().enumerate() {
                    if costs[i] != DUMMY {
                        continue;
                    }

                    match ss {
                        'B' => bh.push((cur.0 - xab, i)),
                        'C' => bh.push((cur.0 - xac, i)),
                        _ => {}
                    }
                }

                warp_from_a_inserted = true;
            }
            'B' => {
                for (i, ss) in s.iter().enumerate() {
                    if costs[i] != DUMMY {
                        continue;
                    }

                    match ss {
                        'A' => bh.push((cur.0 - xab, i)),
                        'C' => bh.push((cur.0 - xbc, i)),
                        _ => {}
                    }
                }

                warp_from_b_inserted = true;
            }
            'C' => {
                for (i, ss) in s.iter().enumerate() {
                    if costs[i] != DUMMY {
                        continue;
                    }

                    match ss {
                        'A' => bh.push((cur.0 - xac, i)),
                        'B' => bh.push((cur.0 - xbc, i)),
                        _ => {}
                    }
                }

                warp_from_c_inserted = true;
            }
            _ => unreachable!(),
        }
    }
    // println!("{:?}", costs);

    println!("{}", costs[n - 1]);
}
