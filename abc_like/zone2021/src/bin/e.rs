// TLE/MLE

use proconio::input;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
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

fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| std::usize::MAX / 2).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State { cost: 0, position: start });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] { continue; }

        for edge in &adj_list[position] {
            let next = State { cost: cost + edge.cost, position: edge.node };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
                // ごまかせなかった
                if next.position == adj_list.len() {
                    return dist;
                }
            }
        }
    }

    dist
}

fn hw_to_1d(cur: (usize, usize), h: usize, w: usize) -> usize {
    w * cur.0 + cur.1
}

fn main() {
    input! {
        r: usize,
        c: usize,
        aa: [[usize; c - 1]; r],
        bb: [[usize; c]; r - 1],
    }

    let mut graph = vec![vec![]; r * c];
    // println!("{}", graph.len());
    for i in 0..r {
        for j in 0..c {
            let cur_hw = hw_to_1d((i, j), r, c);
            // println!("{} {} => {}", i, j, cur_hw);
            // c + 1
            if j + 1 < c {
                let curp = hw_to_1d((i, j + 1), r, c);
                // println!("  c + 1: {}", curp);
                graph[cur_hw].push(Edge { node: curp, cost: aa[i][j] });
            }
            // c - 1
            if j as isize - 1 >= 0 {
                // println!("  c - 1: {}", hw_to_1d((i, j - 1), r, c));
                graph[cur_hw].push(Edge { node: hw_to_1d((i, j - 1), r, c), cost: aa[i][j - 1] });
            }
            // r + 1
            if i + 1 < r {
                // println!("  r + 1: {}", hw_to_1d((i + 1, j), r, c));
                graph[cur_hw].push(Edge { node: hw_to_1d((i + 1, j), r, c), cost: bb[i][j] });
            }
            for k in 0..i {
                let curp = hw_to_1d((k, j), r, c);
                // println!("  k: {}, {}", k, curp);
                graph[cur_hw].push(Edge { node: curp, cost: 1 + i - k });
            }
        }
    }

    // for g in &graph {
    //     println!("{:?}", g);
    // }

    let dist = shortest_path(&graph, 0);
    // println!("{:?}", dist);

    // println!("{}", dist.len());
    println!("{}", dist.last().unwrap());
}
