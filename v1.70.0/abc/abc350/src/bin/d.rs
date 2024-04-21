use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

fn operation_max_num(n: usize) -> usize {
    if n < 3 {
        return 0;
    }

    ((n - 1) + 1) * (n - 1) / 2
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        abm: [(Usize1, Usize1); m],
    }

    // 一直線に友人関係が繋がっていれば, サイズ N の連結成分内では (N-2)+(N-3)+...+1 つ
    // 一直線であろうとなかろうと最終的にできる関係 (グラフ) は同じになるはずで,
    // すると上限値から今ある関係の数を引けばよさそう

    let mut uf = UnionFind::new(n);
    for ab in &abm {
        uf.union(ab.0, ab.1);
    }

    let mut member_num = vec![0; n];
    for i in 0..n {
        member_num[uf.find(i)] += 1;
    }

    let mut edge_num = vec![0; n];
    for ab in &abm {
        edge_num[uf.find(ab.0)] += 1;
    }

    let mut ans = 0usize;
    for i in 0..n {
        if member_num[i] < 3 {
            continue;
        }

        ans += operation_max_num(member_num[i]) - edge_num[i];
    }

    println!("{ans}");
}
