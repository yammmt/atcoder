use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h],
    }

    let mut scores = vec![vec![0; w]; h];
    scores[0][0] = 1;
    // 移動方向が限定されているので BFS を使わずとも解けるはず
    for i in 0..h {
        for j in 0..w {
            if chw[i][j] == '#' || scores[i][j] == 0 {
                continue;
            }

            let next_score = scores[i][j] + 1;
            if i + 1 < h && chw[i + 1][j] == '.' {
                scores[i + 1][j] = scores[i + 1][j].max(next_score);
            }
            if j + 1 < w && chw[i][j + 1] == '.' {
                scores[i][j + 1] = scores[i][j + 1].max(next_score);
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(scores[i][j]);
        }
    }
    println!("{}", ans);
}
