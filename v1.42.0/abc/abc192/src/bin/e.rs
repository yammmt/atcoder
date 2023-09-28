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

// いつそこに行けるかが得られるので Dijkstra が使える…はず
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
        y: usize,
        abtkm: [(usize, usize, usize, usize); m],
    }

    let mut original_edges = vec![vec![]; n + 1];
    for abtk in &abtkm {
        original_edges[abtk.0].push((abtk.1, abtk.2, abtk.3));
        original_edges[abtk.1].push((abtk.0, abtk.2, abtk.3));
    }
    // println!("{:?}", original_edges);

    let mut dist = vec![std::usize::MAX; n + 1];
    let mut heap = BinaryHeap::new();

    heap.push(State { cost: 0, position: x });
    while let Some(State { cost, position }) = heap.pop() {
        // println!("{:?}", heap);
        if cost > dist[position] { continue; }

        dist[position] = cost;
        for e in &original_edges[position] {
            // println!("e: {:?}", e);
            let next_time = if cost % e.2 == 0 {
                cost + e.1
            } else {
                cost + e.1 + e.2 - (cost % e.2)
            };

            if next_time < dist[e.0] {
                heap.push(State { cost: next_time, position: e.0 });
            }
        }
    }
    // println!("{:?}", dist);

    if dist[y] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[y]);
    }
}
