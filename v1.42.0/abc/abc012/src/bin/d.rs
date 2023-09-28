// 問題に対して日本語がとても難しい

use proconio::input;

const UNVISITED: usize = std::usize::MAX / 4;

fn main() {
    input! {
        n: usize,
        m: usize,
        abtm: [(usize, usize, usize); m],
    }

    let mut dist = vec![vec![UNVISITED; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    for abt in &abtm {
        dist[abt.0 - 1][abt.1 - 1] = abt.2;
        dist[abt.1 - 1][abt.0 - 1] = abt.2;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] =  dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = UNVISITED;
    for i in 0..n {
        let mut cur = 0;
        for j in 0..n {
            cur = cur.max(dist[i][j]);
        }
        ans = ans.min(cur);
    }

    println!("{}", ans);
}
