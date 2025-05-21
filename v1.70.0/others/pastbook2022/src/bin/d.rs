use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

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
    // log2n > 10^5 を満たす数でよいが, これなら入力サイズ気にしないし十分間に合うので
    const DOUBLING_LVL_NUM: usize = 64;
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
    input! {
        n: usize,
        abn: [(Usize1, Usize1); n - 1],
        q: usize,
        uvq: [(Usize1, Usize1); q],
    }
    let mut edges = vec![vec![]; n];
    for (a, b) in abn {
        edges[a].push(b);
        edges[b].push(a);
    }

    // ancestor[k][v]: 頂点 v から親方向に 2^k ステップ辿ったところにある頂点の番号
    // ancestor[k][v] = ancestor[k-1][ancestor[k-1][v]], 4 世代前は 2 世代前の 2 世代前

    let lca = Lca::new(n, edges);
    for (u, v) in uvq {
        println!("{}", lca.query(u, v) + 1);
    }
}
