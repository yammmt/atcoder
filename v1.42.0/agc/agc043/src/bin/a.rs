// -> 20.5min

use proconio::input;
use proconio::marker::Chars;

const DUMMY: usize = std::usize::MAX / 4;

fn main() {
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }

    // 通るマスを決め打ちしてやれば反転すべきか否かわかるが 200C100 は全列挙不可
    // 行き止まり次第反転の仕方を全通り試すでも TLE

    // 経路上で今と違う色のマスに入る際にスコアが +1 されるだけでは？
    let mut scores = vec![vec![DUMMY; w]; h];
    scores[0][0] = if shw[0][0] == '#' { 1 } else { 0 };
    for i in 0..h {
        for j in 0..w {
            // 配る DP
            if i + 1 < h {
                scores[i + 1][j] = if shw[i][j] == '.' && shw[i + 1][j] == '#' {
                    (scores[i][j] + 1).min(scores[i + 1][j])
                } else {
                    scores[i][j].min(scores[i + 1][j])
                };
            }
            if j + 1 < w {
                scores[i][j + 1] = if shw[i][j] == '.' && shw[i][j + 1] == '#' {
                    (scores[i][j] + 1).min(scores[i][j + 1])
                } else {
                    scores[i][j].min(scores[i][j + 1])
                };
            }
        }
    }
    // println!("{:?}", scores);

    println!("{}", scores[h - 1][w - 1]);
}
