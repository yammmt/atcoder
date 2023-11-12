use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        chw: [Chars; h],
    }

    // dp[i][j]: (i, j) 到着時の最多経由マス
    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    // 配る
    for i in 0..h {
        let i_nxt = i + 1;
        for j in 0..w {
            if dp[i][j] == 0 {
                continue;
            }

            let j_nxt = j + 1;
            if i_nxt < h && chw[i_nxt][j] == '.' {
                dp[i_nxt][j] = dp[i_nxt][j].max(dp[i][j] + 1);
            }
            if j_nxt < w && chw[i][j_nxt] == '.' {
                dp[i][j_nxt] = dp[i][j_nxt].max(dp[i][j] + 1);
            }
        }
    }

    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            ans = ans.max(dp[i][j]);
        }
    }

    println!("{ans}");
}
