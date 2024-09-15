use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        uvtm: [(Usize1, Usize1, usize); m],
        q: usize,
    }

    // 島数 400 に対して橋が 200,000
    // 多重辺は通過指定がなければ無視してよいので
    // 事実上の橋数は 79,800 程度
    // Dijkstra の計算量は高々 10^5 log (10^5) かそこら

    // 通過指定される橋は 5 個
    // 始点終点を入れると 6 回 Dijkstra をすると求まる
    // 5 個の橋の通過順を総当りすると 120 通り
    // 上の Dijkstra と合わせて 10^7 log (10^5) かそこら, TLE っぽい...
    // これに橋を渡る向きで +2

    // Warshall-Floyd だと島数三乗で 64,000,000 となりギリギリ間に合いそう
    // が, 途中の経路情報を体よく更新できず. 途中に通るものが島であればこれでよさそうだが

    // 時間制限が 4 s だし WF + 渡る向き考えつつの順番全列挙でよいのではないか

    // WF
    let mut cost_min = vec![vec![DUMMY; n]; n];
    for &(u, v, t) in &uvtm {
        cost_min[u][v] = cost_min[u][v].min(t);
        cost_min[v][u] = cost_min[v][u].min(t);
    }
    for i in 0..n {
        cost_min[i][i] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                cost_min[i][j] = cost_min[i][j].min(cost_min[i][k] + cost_min[k][j]);
            }
        }
    }

    for _ in 0..q {
        input! {
            k: usize,
            bk: [Usize1; k],
        }

        let mut ans = DUMMY;
        for perm in (0..k).permutations(k) {
            let mut v_order = vec![];
            for i in 0..k {
                v_order.push(bk[perm[i]]);
            }
            // println!("{:?}", v_order);

            // 目的の橋手前の u or v で停止して渡る
            let mut start_0 = 0;
            let mut start_1 = 0;
            let mut cost_0 = 0;
            let mut cost_1 = 0;
            for &v in &v_order {
                let v0 = uvtm[v].0;
                let v1 = uvtm[v].1;
                // println!("  {start_0}, {start_1} -> {v0}, {v1}");
                let cost_0_cur = (cost_0 + cost_min[start_0][v1])
                    .min(cost_1 + cost_min[start_1][v1])
                    + uvtm[v].2;
                let cost_1_cur = (cost_0 + cost_min[start_0][v0])
                    .min(cost_1 + cost_min[start_1][v0])
                    + uvtm[v].2;
                start_0 = v0;
                start_1 = v1;
                cost_0 = cost_0_cur;
                cost_1 = cost_1_cur;
            }
            cost_0 += cost_min[start_0][n - 1];
            cost_1 += cost_min[start_1][n - 1];

            // println!("  cost: {cost_0}, {cost_1}");
            ans = ans.min(cost_0);
            ans = ans.min(cost_1);
        }
        println!("{ans}");
        // return;
    }
}
