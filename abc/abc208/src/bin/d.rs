// 公式をただ覚えているだけでは詰まるという意味での良問

use proconio::input;

static DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, usize); m],
    }

    // 愚直ワーシャルフロイドでは N <= 400 に対し O(N^3) を N 回回せば良い. 無理
    // と思いきや更新順を考えるとやるだけだった
    let mut dist = vec![vec![DUMMY; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for abc in &abcm {
        dist[abc.0 - 1][abc.1 - 1] = dist[abc.0 - 1][abc.1 - 1].min(abc.2);
    }

    let mut ans = 0;
    for k in 0..n {
        // println!("k: {}", k);
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }

        for i in 0..n {
            for j in 0..n {
                if dist[i][j] == DUMMY {
                    continue;
                }

                ans += dist[i][j];
            }
        }
    }

    println!("{}", ans);
}
