// -*- coding:utf-8-unix -*-

use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        v: [u32; n],
    }

    let mut hm_odd = HashMap::new();
    let mut hm_even = HashMap::new();
    for i in 0..v.len() {
        if i % 2 == 0 {
            let cnt = hm_even.entry(v[i]).or_insert(0);
            *cnt += 1;
        } else {
            let cnt = hm_odd.entry(v[i]).or_insert(0);
            *cnt += 1;
        }
    }

    let mut odd_rank = vec![(0, 0), (0, 0)];
    for (k, v) in &hm_odd {
        if *v > odd_rank[0].1 {
            odd_rank[1] = odd_rank[0];
            odd_rank[0] = (*k, *v);
        } else if *v > odd_rank[1].1 {
            odd_rank[1] = (*k, *v);
        }
    }
    let mut even_rank = vec![(0, 0), (0, 0)];
    for (k, v) in &hm_even {
        if *v > even_rank[0].1 {
            even_rank[1] = even_rank[0];
            even_rank[0] = (*k, *v);
        } else if *v > even_rank[1].1 {
            even_rank[1] = (*k, *v);
        }
    }

    let mut replaced_num_max = 0;
    for i in &odd_rank {
        for j in &even_rank {
            if i.0 == j.0 {
                continue;
            }

            replaced_num_max = replaced_num_max.max(i.1 + j.1);
        }
    }
    println!("{}", n - replaced_num_max);
}
