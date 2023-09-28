// :fu: 21-06 水色優先

// editional "注意深く整理する" の行間が読めず

// floor を無視して f(t) = t + c + d / (t + 1) を微分すると (t + 1) ^ 2 = d が得られ
// t = d.sqrt() - 1 となる 後は誤差や無視した floor を考慮して数通り試す

use proconio::input;
use std::collections::BinaryHeap;

static DUMMY: isize = std::isize::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcdm: [(usize, usize, isize, isize); m],
    }

    let mut edges = vec![vec![]; n];
    for abcd in &abcdm {
        edges[abcd.0 - 1].push((abcd.1 - 1, (abcd.2, abcd.3)));
        edges[abcd.1 - 1].push((abcd.0 - 1, (abcd.2, abcd.3)));
    }

    let mut dist = vec![DUMMY; n];
    let mut heap = BinaryHeap::new();
    // cost, vertex
    heap.push((0, 0));
    while let Some(cur) = heap.pop() {
        // println!("{}", cur.1);
        // println!("  cur: {:?}", cur);
        // println!("  {:?}", heap);
        if dist[cur.1] != DUMMY {
            // println!("  continue");
            continue;
        }

        dist[cur.1] = -cur.0;
        for e in &edges[cur.1] {
            if dist[e.0] != DUMMY {
                continue;
            }

            let mut candidates = vec![];
            let t_f = ((e.1).1 as f64).sqrt() - 1.0;
            for d in -1..2 {
                let cur_t = (t_f - d as f64).ceil();
                // println!("cur_t: {}", cur_t);
                if cur_t >= -cur.0 as f64 {
                    let cur_t = cur_t as isize;
                    let cur_dist = cur_t + (e.1).0 + (e.1).1 / (cur_t + 1);
                    candidates.push((-cur_dist, e.0));
                } else {
                    let cur_t = -cur.0;
                    let cur_dist = cur_t + (e.1).0 + (e.1).1 / (cur_t + 1);
                    candidates.push((-cur_dist, e.0));
                }
            }
            heap.push(*candidates.iter().max().unwrap());
        }
    }
    // println!("{:?}", dist);

    println!(
        "{}",
        if dist[n - 1] == DUMMY {
            -1
        } else {
            dist[n - 1]
        }
    );
}
