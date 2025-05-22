use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;

#[derive(Debug)]
#[allow(dead_code)]
struct Lca {
    n: usize,
    // edge は初期化後には使う機会ないけれど
    edge: Vec<Vec<usize>>,
    depth: Vec<usize>,
    parent: Vec<Vec<usize>>,
}

impl Lca {
    // log2n > 10^5 を満たす数でよいはず
    const DOUBLING_LVL_NUM: usize = 17;
    // Option 型を使うとどうにも遅くなるようなので
    const NO_PARENT: usize = usize::MAX / 3;

    fn new(n: usize, edge: Vec<Vec<usize>>) -> Self {
        // dfs で上書きするのでここでは親 0 でよい
        let mut depth = vec![0; n];
        let mut parent = vec![vec![Self::NO_PARENT; n]; Self::DOUBLING_LVL_NUM];
        Self::dfs(0, 0, 0, &edge, &mut depth, &mut parent);

        // parent を伸ばす, 2^d 先の親を埋める
        for i in 1..Self::DOUBLING_LVL_NUM {
            for j in 0..n {
                if parent[i - 1][j] == Self::NO_PARENT {
                    continue;
                }

                parent[i][j] = parent[i - 1][parent[i - 1][j]];
            }
        }

        Lca {
            n,
            edge,
            depth,
            parent,
        }
    }

    fn query(&self, a: usize, b: usize) -> usize {
        // depth[u] > depth[v]
        let (mut u, mut v) = if self.depth[a] < self.depth[b] {
            (b, a)
        } else {
            (a, b)
        };

        // 二頂点の深さを揃える
        for i in 0..Self::DOUBLING_LVL_NUM {
            // ビットシフト:
            //   depth の差が二進で 10101 であるとして,
            //   ビットシフトなしで初回判定を通すと depth の差は 10100 に更新される.
            //   これでビットシフトを入れなければ, 以後の判定が壊れる.
            if ((self.depth[u] - self.depth[v]) >> i) % 2 == 1 {
                u = self.parent[i][u];
            }
        }

        // LCA を求める
        if u == v {
            u
        } else {
            for i in (0..Self::DOUBLING_LVL_NUM).rev() {
                // 先に深さを揃えたため, parent[i][u] と [i][v] は,
                // 片方が DUMMY ならもう片方も DUMMY になる.
                // 故に DUMMY 判定は片方だけでよい.
                if self.parent[i][u] != Self::NO_PARENT && self.parent[i][u] != self.parent[i][v] {
                    u = self.parent[i][u];
                    v = self.parent[i][v];
                }
            }
            self.parent[0][u]
        }
        // println!("u: {u}, v: {v}, lca: {lca}");
    }

    fn dfs(
        v: usize,
        v_from: usize,
        depth_cur: usize,
        edge: &Vec<Vec<usize>>,
        depth: &mut Vec<usize>,
        parent: &mut Vec<Vec<usize>>,
    ) {
        parent[0][v] = if v == 0 {Self::NO_PARENT} else {v_from};
        depth[v] = depth_cur;
        for &v_nxt in &edge[v] {
            if v_nxt != v_from {
                Self::dfs(v_nxt, v, depth_cur + 1, edge, depth, parent);
            }
        }
    }
}

#[fastout]
fn main() {
    const DUMMY: usize = usize::MAX;
    input! {
        n: usize,
        q: usize,
        abn: [(Usize1, Usize1); n - 1],
        mut uvcq: [(Usize1, Usize1, usize); q],
    }
    let mut edges = vec![vec![]; n];
    for &(a, b) in &abn {
        edges[a].push(b);
        edges[b].push(a);
    }
    let mut edge_to_id = HashMap::new();
    for (i, &(a, b)) in abn.iter().enumerate() {
        edge_to_id.insert((a, b), i);
        edge_to_id.insert((b, a), i);
    }
    uvcq.reverse();

    let lca = Lca::new(n, edges);
    let mut uf = UnionFind::new(n);
    // FIXME:
    let mut grp_root = (0..n).collect::<Vec<usize>>();

    // (u, v) を塗る, を, (u, x), (x, v) を塗る, に読み替える
    // UnionFind に合わせて各グループの最も根に近い頂点を管理する

    let mut ans = vec![DUMMY; n - 1];
    for (mut u, mut v, c) in uvcq {
        let x = lca.query(u, v);
        let depth_x = lca.depth[x];
        let mut depth_u = lca.depth[u];
        let mut depth_v = lca.depth[v];
        while depth_u > depth_x {
            // FIXME:
            let p = lca.parent[0][u];
            let eid = edge_to_id.get(&(u, p)).unwrap();
            ans[eid] = c;
            let ug_old = uf.find(u);
            let ud_old = lca.depth[grp_root[ug_old]];
            let xg_old = uf.find(x);
            let xd_old = lca.depth[grp_root[xg_old]];
            uf.union(u, x);
            // 新たに代表になったグループに, 根に近い方の代表番号を与える
            if uf.find(u) == u_old {
                grp_root[] = grp_root[];
                depth_u = lca.depth[];
            } else {
            }
        }

        while depth_v > depth_x {
            // FIXME:
        }
    }

    for a in ans {
        println!("{a}");
    }
}
