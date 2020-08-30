// -*- coding:utf-8-unix -*-

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
        m: usize,
    }

    // constraint
    if m == 0 {
        println!("1");
        return;
    }

    let mut grp = 0;
    let mut vgrp = vec![0; n + 1];
    let mut vparent = vec![];
    for i in 0..n + 1 {
        vparent.push(i);

    }
    // let idx = vparent.clone();
    let mut vgrpnum = vec![0; n + 1];
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        // println!("a, b: {}, {}", a, b);
        if vgrp[a] == 0 && vgrp[b] == 0 {
            // new
            grp += 1;
            vgrp[a] = grp;
            vgrp[b] = grp;
            vgrpnum[grp as usize] = 2;
        } else {
            let root_a = root(&vparent, vgrp[a]);
            let root_b = root(&vparent, vgrp[b]);
            if root_a == 0 && root_b != 0 {
                // add a to group b
                vgrp[a] = root_b;
                vgrpnum[root_b] += 1;
            } else if root_a != 0 && root_b == 0 {
                // add b to group a
                vgrp[b] = root_a;
                vgrpnum[root_a] += 1;
            } else if root_a != root_b {
                if vgrpnum[root_b] < vgrpnum[root_a] {
                    // merge b to a
                    vgrp[b] = root_a;
                    vparent[root_b] = root_a;
                    vgrpnum[root_a] += vgrpnum[root_b];
                    vgrpnum[root_b] = 0;
                } else {
                    // merge a to b
                    vgrp[a] = root_b;
                    vparent[root_a] = root_b;
                    vgrpnum[root_b] += vgrpnum[root_a];
                    vgrpnum[root_a] = 0;
                }
            }
        }
        // println!("idx:     {:2?}", idx);
        // println!("vgrp:    {:2?}", vgrp);
        // println!("vparent: {:2?}", vparent);
        // println!("vgrpnum: {:2?}\n", vgrpnum);
    }

    let mut ans = 0;
    for i in 0..vgrpnum.len() {
        ans = ans.max(vgrpnum[i]);
    }
    println!("{}", ans);
}
