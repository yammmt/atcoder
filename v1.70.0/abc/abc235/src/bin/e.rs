use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcm: [(Usize1, Usize1, usize); m],
        uvwq: [(Usize1, Usize1, usize); q],
    }

    // 辺の重みがすべて異なるので, MST は一意に定まる
    // MST を先に求めておく
    // 追加された二点間のコストが追加前の二点間のコストより小さければ
    // Yes となる可能性がある？ (必要条件) or Yes と言っていい？ (必要十分条件)
    // 前者っぽい, 直列 1-1-1-1 を始点終点結んで 2-1-1-1 にすると MST でなくなる

    // LCA で到達寸前の一辺が消せるか判定すればよさそう
    // - LCA が u でも v でもない => u, v 到着直前の一辺 x2 と追加辺の重み比較
    // - LCA が u か v => u or v 到着直前の一辺と追加辺の重み比較
    // として実装重い気もするがこんなもんだったっけか
    // FIXME: この方針なら到着直前に拘る必要はなく, パス上の最長辺を見る
    // editorial の方針取れればそんなに実装重くなかった...

    // MST を求める, MST に含まれない辺は以後無視する
    // 二点間を直接結ぶ辺の重みは map に記憶しておく
    let mut uf = UnionFind::new(n);
    let mut member_num = vec![1; n];
    let mut heap = BinaryHeap::new();
    for &(a, b, c) in &abcm {
        // cost, (a, b), query?
        heap.push((Reverse(c), (a, b), None));
    }
    for (i, &(u, v, w)) in uvwq.iter().enumerate() {
        heap.push((Reverse(w), (u, v), Some(i)));
    }

    let mut ans = vec![false; q];
    while let Some((Reverse(_c), (a, b), ans_i)) = heap.pop() {
        if uf.equiv(a, b) {
            continue;
        }

        if let Some(i) = ans_i {
            ans[i] = true;
            continue;
        }

        // TODO: equiv 走る分無駄な定数倍っぽいが
        let grp_a = uf.find(a);
        let grp_b = uf.find(b);
        let member_num_a_before = member_num[grp_a];
        let member_num_b_before = member_num[grp_b];
        uf.union(a, b);
        if uf.find(a) == grp_a {
            member_num[grp_a] += member_num_b_before;
            member_num[grp_b] = 0;
        } else {
            member_num[grp_b] += member_num_a_before;
            member_num[grp_a] = 0;
        }
        if member_num[grp_a] == n || member_num[grp_b] == n {
            // なくともよいが
            break;
        }
    }

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
