// :fu: 21-08 わからん むずくない？

use proconio::input;

fn root(v: &Vec<usize>, k: usize) -> usize {
    if v[k] == k {
        k
    } else {
        root(&v, v[k])
    }
}

fn main() {
    input! {
        n: usize,
        uvwn1: [(usize, usize, usize); n - 1],
    }
    let mut edges = vec![];
    for uvw in &uvwn1 {
        edges.push((uvw.2, (uvw.0 - 1, uvw.1 - 1)));
    }
    edges.sort_unstable();

    let mut ans = 0;
    let mut vgrp = (0..n).collect::<Vec<usize>>();
    let mut vparent = (0..n).collect::<Vec<usize>>();
    let mut vmembernum = vec![1; n];
    for e in &edges {
        let root_a = root(&vparent, vgrp[(e.1).0]);
        let root_b = root(&vparent, vgrp[(e.1).1]);
        ans += e.0 * vmembernum[root_a] * vmembernum[root_b];

        if root_a == root_b {
            continue;
        } else if root_a == 0 && root_b != 0 {
            // add a to group b
            vgrp[(e.1).0] = root_b;
            vmembernum[root_b] += 1;
        } else if root_a != 0 && root_b == 0 {
            vgrp[(e.1).1] = root_a;
            vmembernum[root_a] += 1;
        } else {
            // 小さい方を大きい方にマージする
            if vmembernum[root_a] > vmembernum[root_b] {
                vgrp[(e.1).1] = root_a;
                vparent[root_b] = root_a;
                vmembernum[root_a] += vmembernum[root_b];
                vmembernum[root_b] = 0;
                // println!("m: {:?}", vmembernum);
            } else {
                vgrp[(e.1).0] = root_b;
                vparent[root_a] = root_b;
                vmembernum[root_b] += vmembernum[root_a];
                vmembernum[root_a] = 0;
                // println!("m: {:?}", vmembernum);
            }
        }
    }

    println!("{}", ans);
}
