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
        abn: [(usize, usize); n],
    }
    let maxab = 400001;

    let mut vgrp = vec![0; maxab];
    let mut vparent = (0..maxab).collect::<Vec<usize>>();
    let mut vmembernum = vec![0; maxab];
    let mut grpnum = 1;
    for i in 0..n {
        if vgrp[abn[i].0] == 0 && vgrp[abn[i].1] == 0 {
            // create new group
            vgrp[abn[i].0] = grpnum;
            vgrp[abn[i].1] = grpnum;
            vmembernum[grpnum] = if abn[i].0 == abn[i].1 {
                1
            } else {
                2
            };
            grpnum += 2;
        } else {
            let root_a = root(&vparent, vgrp[abn[i].0]);
            let root_b = root(&vparent, vgrp[abn[i].1]);
            if root_a == root_b {
                continue;
            } else if root_a == 0 && root_b != 0 {
                // add a to group b
                vgrp[abn[i].0] = root_b;
                vmembernum[root_b] += 1;
            } else if root_a != 0 && root_b == 0 {
                vgrp[abn[i].1] = root_a;
                vmembernum[root_a] += 1;
            } else {
                // 小さい方を大きい方にマージする
                if vmembernum[root_a] > vmembernum[root_b] {
                    vgrp[abn[i].1] = root_a;
                    vparent[root_b] = root_a;
                    vmembernum[root_a] += vmembernum[root_b];
                    vmembernum[root_b] = 0;
                    // println!("m: {:?}", vmembernum);
                } else {
                    vgrp[abn[i].0] = root_b;
                    vparent[root_a] = root_b;
                    vmembernum[root_b] += vmembernum[root_a];
                    vmembernum[root_a] = 0;
                    // println!("m: {:?}", vmembernum);
                }
            }
        }
    }
    // println!("vmembernum: {:?}", vmembernum);

    let mut vrootgrp = vec![0; maxab];
    for i in 1..maxab {
        vrootgrp[i] = root(&vparent, vgrp[i]);
    }

    let mut vcardnum = vec![0; maxab];
    for ab in &abn {
        vcardnum[vrootgrp[ab.0]] += 1;
    }
    // println!("vcardnum: {:?}", vcardnum);

    let mut ans = 0;
    for i in 1..maxab {
        ans += vmembernum[i].min(vcardnum[i]);
    }
    println!("{}", ans);
}
