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
        other
            .cost
            .cmp(&self.cost)
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
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| std::usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    dist
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, usize); m],
    }

    let mut graph = vec![vec![]; n];
    for abc in &abcm {
        graph[abc.0 - 1].push(Edge {
            node: abc.1 - 1,
            cost: abc.2,
        });
        graph[abc.1 - 1].push(Edge {
            node: abc.0 - 1,
            cost: abc.2,
        });
    }

    let from_begin = shortest_path(&graph, 0);
    let from_end = shortest_path(&graph, n - 1);
    for i in 0..n {
        println!("{}", from_begin[i] + from_end[i]);
    }
}
