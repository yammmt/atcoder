// :fu: :fu: :fu: 21-04 Dijkstra のデバッグがほんとだめ

use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

#[allow(clippy::ptr_arg)]
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize, xq: &Vec<usize>) -> Vec<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| std::usize::MAX).collect();
    let mut date_idx = 0;

    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, position: start });
    let mut next_date_pathes = vec![];

    while let Some(State { cost, position }) = heap.pop() {
        // println!("cost: {}, position: {}", cost, position);
        // println!("day: {}, allowed_cost: {}", date_idx, xq[date_idx]);
        // println!("  h: {:?}", heap);
        // println!("  n: {:?}", next_date_pathes);
        if date_idx >= dist[position] {
            // もう到達しているので無視
            // 空になるなら next_date_pathes 押す
            if heap.is_empty() {
                if date_idx == xq.len() - 1 {
                    // 残りすべて到着不可
                    break;
                }

                date_idx += 1;
                for ndp in &next_date_pathes {
                    heap.push(*ndp);
                }
                next_date_pathes.clear();
            }
            continue;
        }

        if xq[date_idx] < cost {
            // コスト不足で通れない
            if date_idx == xq.len() - 1 {
                // 残りすべて到着不可
                break;
            }

            date_idx += 1;
            heap.push(State { cost, position });
            for ndp in &next_date_pathes {
                heap.push(*ndp);
            }
            next_date_pathes.clear();
            continue;
        }

        dist[position] = date_idx;
        // println!("  dist[{}] = {}", position, date_idx);
        for edge in &adj_list[position] {
            let next = State { cost: edge.cost, position: edge.node };

            if dist[next.position] != std::usize::MAX {
                continue;
            }

            next_date_pathes.push(next);
        }
        if heap.is_empty() {
            // 初回分だけは先に足して置かなければ 0 日目の判定がバグる！
            if position == 0 {
                for ndp in &next_date_pathes {
                    heap.push(*ndp);
                }
                next_date_pathes.clear();
            } else {
                if date_idx == xq.len() - 1 {
                    // 残りすべて到着不可
                    break;
                }

                date_idx += 1;
                for ndp in &next_date_pathes {
                    heap.push(*ndp);
                }
                next_date_pathes.clear();
            }
        }
    }

    dist
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcm: [(usize, usize, usize); m],
        xq: [usize; q],
    }

    let mut graph = vec![vec![]; n];
    for abc in &abcm {
        graph[abc.0 - 1].push(Edge { node: abc.1 - 1, cost: abc.2 });
        graph[abc.1 - 1].push(Edge { node: abc.0 - 1, cost: abc.2 });
    }

    let from_0 = shortest_path(&graph, 0, &xq);
    // println!("{:?}", from_0);

    let mut ans = vec![0; q];
    for f in &from_0 {
        if *f > ans.len() {
            continue;
        }
        ans[*f] += 1;
    }

    let mut cur = 0;
    for a in &ans {
        cur += *a;
        println!("{}", cur);
    }
}
