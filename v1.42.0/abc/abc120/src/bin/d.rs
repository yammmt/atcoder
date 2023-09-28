// 35.5min  UF ライブラリ辛い...

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
        mut abm: [(usize, usize); m],
    }
    abm.reverse();

    // 逆に UnionFind まではしてグループの島数判定で TLE 喰らうパターン多そう
    // 繋がっている島の組数
    let mut ans = vec![0];

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
            ans.push(ans[i] + 1);
            // println!("g {:?}", ans);
        } else {
            let root_a = root(&vparent, vgrp[abm[i].0]);
            let root_b = root(&vparent, vgrp[abm[i].1]);
            let grp_a_num_before = vmembernum[root_a];
            let grp_b_num_before = vmembernum[root_b];
            if root_a == root_b {
                ans.push(ans[i]);
                continue;
            } else if root_a == 0 && root_b != 0 {
                // add a to group b
                vgrp[abm[i].0] = root_b;
                vmembernum[root_b] += 1;
                ans.push(
                    ans[i]
                    + (vmembernum[root_b] * (vmembernum[root_b] - 1)) / 2
                    - ((vmembernum[root_b] - 1) * (vmembernum[root_b] - 2)) / 2
                );
            } else if root_a != 0 && root_b == 0 {
                vgrp[abm[i].1] = root_a;
                vmembernum[root_a] += 1;
                ans.push(
                    ans[i]
                    + (vmembernum[root_a] * (vmembernum[root_a] - 1)) / 2
                    - ((vmembernum[root_a] - 1) * (vmembernum[root_a] - 2)) / 2
                );
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
                let root_a = root(&vparent, vgrp[abm[i].0]);
                let grp_ab_num_after = vmembernum[root_a];
                ans.push(
                    ans[i]
                    + (grp_ab_num_after * (grp_ab_num_after - 1)) / 2
                    - (grp_a_num_before * (grp_a_num_before - 1)) / 2
                    - (grp_b_num_before * (grp_b_num_before - 1)) / 2
                );
            }
        }
    }
    // println!("{:?}", ans);

    ans.reverse();
    let maxcmb = (n * (n - 1)) / 2;
    for a in ans.iter().skip(1) {
        println!("{}", maxcmb - a);
    }
}
