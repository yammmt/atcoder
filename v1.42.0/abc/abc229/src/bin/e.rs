use proconio::input;

fn root(mut v: &mut Vec<usize>, k: usize) -> usize {
    if v[k] == k {
        k
    } else {
        let vk = v[k];
        v[k] = root(&mut v, vk);
        v[k]
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(usize, usize); m],
    }

    let mut edges = vec![vec![]; n];
    for ab in &abm {
        // 制約より ab.0 < ab.1
        edges[ab.0 - 1].push(ab.1 - 1);
    }

    // 尻から順につけていく
    let mut vgrp = vec![0; n + 1];
    let mut vparent = (0..n + 1).collect::<Vec<usize>>();
    let mut vmembernum = vec![0; n + 1];
    vmembernum[0] = n;
    let mut cur_group_id = 1;
    let mut cur_group_num = 0;

    let mut ans = vec![];
    for i in (0..n).rev() {
        // println!("i: {}", i);
        ans.push(cur_group_num);

        // 一人グループを作る
        vgrp[i] = cur_group_id;
        vmembernum[0] -= 1;
        vmembernum[cur_group_id] += 1;
        cur_group_num += 1;

        cur_group_id += 1;

        for v in &edges[i] {
            // 制約を受けた辺作成部より, 必ず自身より数字の重い頂点への辺を対象とする
            let root_a = root(&mut vparent, vgrp[i]);
            let root_b = root(&mut vparent, vgrp[*v]);
            if root_a == root_b {
                continue;
            }

            // 小さい方を大きい方にマージする
            if vmembernum[root_a] > vmembernum[root_b] {
                vgrp[*v] = root_a;
                vparent[root_b] = root_a;
                vmembernum[root_a] += vmembernum[root_b];
                vmembernum[root_b] = 0;
            } else {
                vgrp[i] = root_b;
                vparent[root_a] = root_b;
                vmembernum[root_b] += vmembernum[root_a];
                vmembernum[root_a] = 0;
            }
            cur_group_num -= 1;
        }
    }

    ans.reverse();
    for a in &ans {
        println!("{}", a);
    }
}
