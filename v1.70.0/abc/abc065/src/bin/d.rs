use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        xyn: [(usize, usize); n],
    }

    // プリム法をと思ったが道の候補が N^2 個あるので TLE
    // 任意の二点間を動けるようにするには N-1 個の道を作ることになりそう
    // 最初の一本については x/y 座標でそれぞれソートしてコスト最小の辺を立てるでよさそう
    // そもそも辺の候補って x/y それぞれソートした隣接領域だけになるのでは？
    // => 連結成分が拾えない, 並行して UF する？よさそう

    // (cost, (v1, v2))
    let mut bts = BTreeSet::new();

    let mut order_by_x = vec![];
    for (i, xy) in xyn.iter().enumerate() {
        order_by_x.push((xy.0, i));
    }
    order_by_x.sort_unstable();
    for i in 1..n {
        let x_pre = order_by_x[i - 1].0;
        let x_cur = order_by_x[i].0;
        let i_pre = order_by_x[i - 1].1;
        let i_cur = order_by_x[i].1;
        bts.insert((x_cur - x_pre, (i_pre, i_cur)));
    }

    let mut order_by_y = vec![];
    for (i, xy) in xyn.iter().enumerate() {
        order_by_y.push((xy.1, i));
    }
    order_by_y.sort_unstable();
    for i in 1..n {
        let y_pre = order_by_y[i - 1].0;
        let y_cur = order_by_y[i].0;
        let i_pre = order_by_y[i - 1].1;
        let i_cur = order_by_y[i].1;
        bts.insert((y_cur - y_pre, (i_pre, i_cur)));
    }

    let mut uf = UnionFind::new(n);
    let mut ans = 0;
    let mut edges_num = 0;
    while let Some((cur_cost, (p1, p2))) = bts.pop_first() {
        if uf.equiv(p1, p2) {
            continue;
        }

        ans += cur_cost;
        edges_num += 1;
        uf.union(p1, p2);
        if edges_num == n - 1 {
            break;
        }
    }

    println!("{ans}");
}
