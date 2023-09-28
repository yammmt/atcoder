// -*- coding:utf-8-unix -*-

// 8min

use permutohedron::heap_recursive;
use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut connected = vec![HashSet::new(); n];
    abm.iter().for_each(|ab| {
        connected[ab.0 - 1].insert(ab.1 - 1);
        connected[ab.1 - 1].insert(ab.0 - 1);
    });
    let mut vertexes = (1..n).collect::<Vec<usize>>();
    let mut ans = 0;
    heap_recursive(&mut vertexes, |p| {
        let mut pass = true;
        for i in 0..n - 1 {
            let v_prev = if i == 0 {
                0
            } else {
                p[i - 1]
            };
            if !connected[v_prev].contains(&p[i]) {
                pass = false;
            }
        }

        if pass {
            ans += 1;
        }
    });

    println!("{}", ans);
}
