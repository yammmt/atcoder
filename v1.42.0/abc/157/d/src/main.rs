// -*- coding:utf-8-unix -*-

// 55min 3WA
// O(n^2) で TLE (19min), グループなし者の処理漏らし (50min), 演算結果 -1

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
        k: usize,
        abm: [(usize, usize); m],
        cdk: [(usize, usize); k],
    }

    let mut vgrp = vec![0; n + 1];
    let mut vparent = (0..n + 1).collect::<Vec<usize>>();
    let mut vmembernum = vec![0; n + 1];
    let mut grpnum = 1;
    for i in 0..m {
        if vgrp[abm[i].0] == 0 && vgrp[abm[i].1] == 0 {
            // create new group
            vgrp[abm[i].0] = grpnum;
            vgrp[abm[i].1] = grpnum;
            vmembernum[grpnum] = 2;
            grpnum += 2;
        } else {
            let root_a = root(&vparent, vgrp[abm[i].0]);
            let root_b = root(&vparent, vgrp[abm[i].1]);
            if root_a == root_b {
                continue;
            } else if root_a == 0 && root_b != 0 {
                // add a to group b
                vgrp[abm[i].0] = root_b;
                vmembernum[root_b] += 1;
            } else if root_a != 0 && root_b == 0 {
                vgrp[abm[i].1] = root_a;
                vmembernum[root_a] += 1;
            } else {
                // 小さい方を大きい方にマージする
                if vmembernum[root_a] > vmembernum[root_b] {
                    vgrp[abm[i].1] = root_a;
                    vparent[root_b] = root_a;
                    vmembernum[root_a] += vmembernum[root_b];
                    vmembernum[root_b] = 0;
                    // println!("m: {:?}", vmembernum);
                } else {
                    vgrp[abm[i].0] = root_b;
                    vparent[root_a] = root_b;
                    vmembernum[root_b] += vmembernum[root_a];
                    vmembernum[root_a] = 0;
                    // println!("m: {:?}", vmembernum);
                }
            }
        }
    }

    let mut root_grp = vec![0; n + 1];
    for i in 0..n + 1 {
        root_grp[i] = root(&vparent, vgrp[i]);
    }

    let mut vfriends = vec![0; n + 1];
    for ab in &abm {
        vfriends[ab.0] += 1;
        vfriends[ab.1] += 1;
    }

    let mut vblocked_in_grp = vec![0; n + 1];
    for cd in &cdk {
        if root_grp[cd.0] != 0 && root_grp[cd.0] == root_grp[cd.1] {
            vblocked_in_grp[cd.0] += 1;
            vblocked_in_grp[cd.1] += 1;
        }
    }


    let mut ans = vec![0; n];
    for i in 1..n + 1 {
        ans[i - 1] = (vmembernum[root_grp[i]] - (vfriends[i] + 1) - vblocked_in_grp[i]).max(0);
    }

    for i in 0..n {
        print!("{}", ans[i]);
        if i < n - 1 {
            print!(" ");
        } else {
            println!();
        }
    }
}
