// 45.5min (TLE: 31min/計算量多い嘘解法)
// WA (TLE): 辺を一から作り直したらだめだった

use proconio::input;

const DUMMY: usize = std::usize::MAX / 3;

fn main() {
    input! {
        n: usize,
        m: usize,
        uvlm: [(usize, usize, usize); m],
    }
    // 制約より多重辺と自己ループはなし
    let mut dist = vec![vec![DUMMY; n]; n];
    let mut dist_from_0 = vec![DUMMY; n];
    for uvl in &uvlm {
        if uvl.0 == 1 {
            dist_from_0[uvl.1 - 1] = uvl.2;
        } else if uvl.1 == 1 {
            dist_from_0[uvl.0 - 1] = uvl.2;
        } else {
            dist[uvl.0 - 1][uvl.1 - 1] = uvl.2;
            dist[uvl.1 - 1][uvl.0 - 1] = uvl.2;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }

    let mut ans = DUMMY;
    for i in 1..n {
        for j in i + 1..n {
            if dist_from_0[i] == DUMMY || dist_from_0[j] == DUMMY {
                continue;
            }

            ans = ans.min(dist[i][j] + dist_from_0[i] + dist_from_0[j]);
        }
    }

    println!(
        "{}",
        if ans == DUMMY {
            -1
        } else {
            ans as isize
        }
    );
}
