// UF のライブラリが悪かったぞ

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
        mut abcm: [(usize, usize, i64); m],
    }
    // コスト昇順 (負の数が先に来るので貪欲に足す)
    abcm.sort_unstable_by(|x, y| x.2.cmp(&y.2));
    let abcm = abcm;

    // 最初にすべての報酬を持っているとして最低限連結になる程度に辺を足していく (最小全域木)
    // 負の辺を取り除くメリットはないので全部繋げておく
    let mut ans = abcm.iter().map(|abc| abc.2).sum::<i64>();

    // UnionFind で全頂点をマークする
    let mut vgrp = (0..n + 1).collect::<Vec<usize>>();
    let mut vparent = (0..n + 1).collect::<Vec<usize>>();
    let mut vmembernum = vec![1; n + 1];

    for abc in &abcm {
        // println!("{:?}", abc);
        let root_a = root(&mut vparent, vgrp[abc.0]);
        let root_b = root(&mut vparent, vgrp[abc.1]);

        if abc.2 > 0 && vmembernum[root_a] == n {
            // 負の辺を使い果たしてもう連結
            break;
        } else if abc.2 > 0 && root_a == root_b {
            // 同じグループ
            continue;
        }

        if root_a == 0 && root_b != 0 {
            // add a to group b
            vgrp[abc.0] = root_b;
            vmembernum[root_b] += 1;
        } else if root_a != 0 && root_b == 0 {
            vgrp[abc.1] = root_a;
            vmembernum[root_a] += 1;
        } else {
            // 小さい方を大きい方にマージする
            if vmembernum[root_a] > vmembernum[root_b] {
                vgrp[abc.1] = root_a;
                vparent[root_b] = root_a;
                vmembernum[root_a] += vmembernum[root_b];
                vmembernum[root_b] = 0;
                // println!("m: {:?}", vmembernum);
            } else {
                vgrp[abc.0] = root_b;
                vparent[root_a] = root_b;
                vmembernum[root_b] += vmembernum[root_a];
                vmembernum[root_a] = 0;
                // println!("m: {:?}", vmembernum);
            }
        }

        ans -= abc.2;
        // println!("  {}", ans);
    }


    println!("{}", ans);
}
