use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            // -> right
            let j_nxt = j + 1;
            if j_nxt < w && ahw[i][j_nxt] == '.' {
                dp[i][j_nxt] = (dp[i][j_nxt] + dp[i][j]) % MOD;
            }

            // -> bottom
            let i_nxt = i + 1;
            if i_nxt < h && ahw[i_nxt][j] == '.' {
                dp[i_nxt][j] = (dp[i_nxt][j] + dp[i][j]) % MOD;
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
