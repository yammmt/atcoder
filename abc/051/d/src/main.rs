// ワーシャルフロイドの経路復元

// 最短経路が複数考えられる場合に全部考慮しないと WA っぽい
// 例えば以下で "0" を返さねばならないらしい
// 3 3
// 1 2 1
// 1 3 2
// 2 3 1

use proconio::input;

const NOT_YET: usize = std::usize::MAX / 3;

fn main() {
    input! {
        n: usize,
        m: usize,
        abcm: [(usize, usize, usize); m],
    }

    let mut dists = vec![vec![NOT_YET; n]; n];
    for i in 0..n {
        dists[i][i] = 0;
    }
    for abc in &abcm {
        dists[abc.0 - 1][abc.1 - 1] = dists[abc.0 - 1][abc.1 - 1].min(abc.2);
        dists[abc.1 - 1][abc.0 - 1] = dists[abc.1 - 1][abc.0 - 1].min(abc.2);
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dists[i][j] = dists[i][j].min(dists[i][k] + dists[k][j]);
            }
        }
    }

    let mut ans = m;
    for abc in &abcm {
        for i in 0..n {
            if dists[i][abc.0 - 1] + abc.2 == dists[i][abc.1 - 1] {
                ans -= 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
