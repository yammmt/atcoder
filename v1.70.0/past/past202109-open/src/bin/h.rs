use proconio::input;
use std::cmp::Ordering;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
        x: usize,
        abcn: [(usize, usize, usize); n - 1],
    }

    let mut edges = vec![vec![]; n];
    for abc in abcn {
        let a = abc.0 - 1;
        let b = abc.1 - 1;
        let c = abc.2;
        edges[a].push((b, c));
        edges[b].push((a, c));
    }

    for i in 0..n {
        // println!("i: {i}");
        let mut bts = BTreeSet::new();
        let mut visited = vec![false; n];
        visited[i] = true;
        for e in &edges[i] {
            match e.1.cmp(&x) {
                Ordering::Greater => {}
                Ordering::Equal => {
                    println!("Yes");
                    return;
                }
                Ordering::Less => {
                    bts.insert((e.1, e.0));
                }
            }
        }

        while let Some(cur) = bts.pop_first() {
            if visited[cur.1] {
                continue;
            }

            visited[cur.1] = true;
            for e in &edges[cur.1] {
                let dist = cur.0 + e.1;
                if visited[e.0] || dist > x {
                    continue;
                }

                if dist == x {
                    println!("Yes");
                    return;
                }

                bts.insert((dist, e.0));
            }
        }
    }

    println!("No");
}
