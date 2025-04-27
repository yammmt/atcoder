// 編集距離 DP

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    const DUMMY_BIG: usize = usize::MAX / 4;
    input! {
        m: usize,
        n: usize,
        s: Chars,
        t: Chars,
    }

    // dp[i][j]: S の最初の i 文字分と T の最初の j 文字分の間の編集距離
    let mut dp = vec![vec![DUMMY_BIG; n + 1]; m + 1];
    dp[0][0] = 0;

    // dp[i][j] を求めるにあたって, S に対しての操作は三通り:
    // - A. T の末尾の文字に変更: dp[i-1][j-1]+(1or0)
    // - B. 末尾の文字を削除: dp[i-1][j]+1
    //   - S[0:i-1] と T[0:j] が等しい
    //   - S[i-2] の時点で T[j-1] まで一致しているのだから消せば良い, との考え, のはず
    // - C. T の末尾の文字を挿入: dp[i][j-1]+1
    //   - S[0:i] と T[0:j-1] が等しい
    //   - T[j-2] までしか一致していないのだから足す, との考え, のはず

    for i in 0..=m {
        for j in 0..=n {
            if i > 0 && j > 0 {
                dp[i][j] =
                    dp[i][j].min(dp[i - 1][j - 1] + if s[i - 1] == t[j - 1] { 0 } else { 1 });
            }
            if i > 0 {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + 1);
            }
            if j > 0 {
                dp[i][j] = dp[i][j].min(dp[i][j - 1] + 1);
            }
        }
    }

    println!(
        "
    {}",
        dp[m][n]
    );
}
