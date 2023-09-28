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
        pn: [u64; n],
        xym: [(usize, usize); m],
    }

    let mut vgrp = vec![0; n + 1];
    let mut vparent = (0..n + 1).collect::<Vec<usize>>();
    let mut vmembernum = vec![0; n + 1];
    let mut grpnum = 1;
    for i in 0..m {
        if vgrp[xym[i].0] == 0 && vgrp[xym[i].1] == 0 {
            // create new group
            vgrp[xym[i].0] = grpnum;
            vgrp[xym[i].1] = grpnum;
            vmembernum[grpnum] = 2;
            grpnum += 2;
        } else {
            let root_a = root(&vparent, vgrp[xym[i].0]);
            let root_b = root(&vparent, vgrp[xym[i].1]);
            if root_a == root_b {
                continue;
            } else if root_a == 0 && root_b != 0 {
                // add a to group b
                vgrp[xym[i].0] = root_b;
                vmembernum[root_b] += 1;
            } else if root_a != 0 && root_b == 0 {
                vgrp[xym[i].1] = root_a;
                vmembernum[root_a] += 1;
            } else {
                // 小さい方を大きい方にマージする
                if vmembernum[root_a] > vmembernum[root_b] {
                    vgrp[xym[i].1] = root_a;
                    vparent[root_b] = root_a;
                    vmembernum[root_a] += vmembernum[root_b];
                    vmembernum[root_b] = 0;
                    // println!("m: {:?}", vmembernum);
                } else {
                    vgrp[xym[i].0] = root_b;
                    vparent[root_a] = root_b;
                    vmembernum[root_b] += vmembernum[root_a];
                    vmembernum[root_a] = 0;
                    // println!("m: {:?}", vmembernum);
                }
            }
        }
    }
    // println!("vgrp: {:?}", vgrp);

    let mut ans = 0;
    for i in 0..n {
        let root_a = root(&vparent, vgrp[i + 1]);
        let root_b = root(&vparent, vgrp[pn[i] as usize]);
        if (root_a != 0 && root_a == root_b) || (root_a == 0 && root_b == 0 && i + 1 == pn[i] as usize) {
            ans += 1;
        }
    }

    println!("{}", ans);
}
