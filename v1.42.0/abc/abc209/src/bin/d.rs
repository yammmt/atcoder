use proconio::input;
use std::collections::VecDeque;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        q: usize,
        abn1: [(usize, usize); n - 1],
        cdq: [(usize, usize); q],
    }
    let mut edges = vec![vec![]; n];
    for ab in &abn1 {
        edges[ab.0 - 1].push(ab.1 - 1);
        edges[ab.1 - 1].push(ab.0 - 1);
    }

    let mut from_0 = vec![DUMMY; n];
    from_0[0] = 0;
    let mut vdq = VecDeque::new();
    vdq.push_back((0, 1));
    while let Some(cur) = vdq.pop_front() {
        for e in &edges[cur.0] {
            if from_0[*e] != DUMMY {
                continue;
            }

            from_0[*e] = cur.1;
            vdq.push_back((*e, cur.1 + 1));
        }
    }

    for cd in &cdq {
        let from_c = from_0[cd.0 - 1] as isize;
        let from_d = from_0[cd.1 - 1] as isize;
        println!(
            "{}",
            if (from_c - from_d).abs() % 2 == 0 {
                "Town"
            } else {
                "Road"
            }
        );
    }
}
