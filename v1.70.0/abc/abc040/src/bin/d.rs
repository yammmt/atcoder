use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut abym: [(Usize1, Usize1, usize); m],
        q: usize,
        vwq: [(Usize1, usize); q],
    }
    // 作成年降順
    abym.sort_unstable_by(|a, b| b.2.cmp(&a.2));
    // 作成年降順
    let mut vwiq = vec![];
    for (i, vw) in vwq.iter().enumerate() {
        vwiq.push((vw.0, vw.1, i));
    }
    vwiq.sort_unstable_by(|a, b| b.1.cmp(&a.1));

    let mut uf = UnionFind::new(n);
    let mut member_num = vec![1; n];
    let mut bridge_idx = 0;
    let mut ans = vec![0; q];
    for (v, w, i) in vwiq {
        // w 年ちょうどの橋は使わない
        while bridge_idx < m && abym[bridge_idx].2 > w {
            let bi = bridge_idx;
            bridge_idx += 1;
            let group_a = uf.find(abym[bi].0);
            let group_b = uf.find(abym[bi].1);
            if group_a == group_b {
                continue;
            }

            let member_a = member_num[group_a];
            let member_b = member_num[group_b];
            uf.union(group_a, group_b);
            let group_a_after = uf.find(abym[bi].0);
            if group_a_after == group_a {
                // b を a にマージ
                member_num[group_a] += member_b;
                member_num[group_b] = 0;
            } else {
                // a を b にマージ
                member_num[group_b] += member_a;
                member_num[group_a] = 0;
            }
        }

        let g = uf.find(v);
        ans[i] = member_num[g];
    }
    // println!("{:?}", uf);

    for a in ans {
        println!("{a}");
    }
}
