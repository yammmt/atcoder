use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(n);
    let mut members = vec![vec![]; n];
    for i in 0..n {
        members[i].push(i);
    }

    for _ in 0..q {
        input! {
            a: usize,
            u: Usize1,
        }

        if a == 1 {
            input! {
                v: Usize1,
            }

            let gu_before = uf.find(u);
            let gv_before = uf.find(v);
            if gu_before == gv_before {
                // これがないと後のメンバー移す処理が無限ループになる
                continue;
            }

            uf.union(gu_before, gv_before);

            let gu_after = uf.find(u);
            if gu_before == gu_after {
                while let Some(m) = members[gv_before].pop() {
                    members[gu_before].push(m);
                }
            } else {
                while let Some(m) = members[gu_before].pop() {
                    members[gv_before].push(m);
                }
            }
        } else {
            let g = uf.find(u);
            // log 分計算量増えるの嫌だが, 出力する総数から考えるには大丈夫なはず
            let mut ans = members[g].clone();
            ans.sort_unstable();
            for (i, a) in ans.iter().enumerate() {
                print!("{}", a + 1);
                if i == ans.len() - 1 {
                    println!();
                } else {
                    print!(" ");
                }
            }
        }
    }
}
