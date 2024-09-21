// LCA の練習問題としてググったら出てきた
// TODO: LCA (ダブリング) をライブラリとして切り出しておく

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const DUMMY: usize = usize::MAX / 3;
// log2n > 10^5 を満たす数でよいが, これなら入力サイズ気にしないし十分間に合うので
const DOUBLING_LVL_NUM: usize = 64;

// for print debug
// const DUMMY: usize = 334;
// const DOUBLING_LVL_NUM: usize = 4;

fn dfs(
    v: usize,
    v_from: usize,
    depth_cur: usize,
    edge: &Vec<Vec<usize>>,
    depth: &mut Vec<usize>,
    parent: &mut Vec<Vec<usize>>,
) {
    parent[0][v] = if v == 0 { DUMMY } else { v_from };
    depth[v] = depth_cur;
    for &v_nxt in &edge[v] {
        if v_nxt != v_from {
            dfs(v_nxt, v, depth_cur + 1, edge, depth, parent);
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xyn: [(Usize1, Usize1); n - 1],
        q: usize,
        abq: [(Usize1, Usize1); q],
    }
    let mut edge = vec![vec![]; n];
    for (x, y) in xyn {
        edge[x].push(y);
        edge[y].push(x);
    }

    let mut depth = vec![DUMMY; n];
    // parent[d][v]: v の 2^d 先の親, n は log2n でよいが面倒
    let mut parent = vec![vec![DUMMY; n]; DOUBLING_LVL_NUM];
    dfs(0, 0, 0, &edge, &mut depth, &mut parent);
    // println!("parent: {:?}", parent);

    // parent を伸ばす, 2^d 先の親を埋める
    for i in 1..DOUBLING_LVL_NUM {
        for j in 0..n {
            if parent[i - 1][j] == DUMMY {
                // もう根に戻った
                continue;
            }

            parent[i][j] = parent[i - 1][parent[i - 1][j]];
        }
    }
    // println!("parent: {:?}", parent);

    for (a, b) in abq {
        // println!("a: {a}, b: {b}");
        // depth[u] > depth[v]
        let (mut u, mut v) = if depth[a] < depth[b] { (b, a) } else { (a, b) };

        // 二頂点の深さを揃える
        for i in 0..DOUBLING_LVL_NUM {
            // ビットシフト:
            //   depth の差が二進で 10101 であるとして,
            //   ビットシフトなしで初回判定を通すと depth の差は 10100 に更新される.
            //   これでビットシフトを入れなければ, 以後の判定が壊れる.
            if ((depth[u] - depth[v]) >> i) % 2 == 1 {
                u = parent[i][u];
            }
        }

        // LCA を求める
        let lca = if u == v {
            u
        } else {
            for i in (0..DOUBLING_LVL_NUM).rev() {
                // 先に深さを揃えたため, parent[i][u] と [i][v] は,
                // 片方が DUMMY ならもう片方も DUMMY になる.
                // 故に DUMMY 判定は片方だけでよい.
                if parent[i][u] != DUMMY && parent[i][u] != parent[i][v] {
                    u = parent[i][u];
                    v = parent[i][v];
                }
            }
            parent[0][u]
        };
        // println!("u: {u}, v: {v}, lca: {lca}");

        // depth(u, LCA(u, v)) + depth(v, LCA(u, v)) が二頂点間の長さになる
        println!("{}", depth[a] + depth[b] - 2 * depth[lca] + 1);
    }
}
