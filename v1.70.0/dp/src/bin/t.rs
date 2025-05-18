use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // dp[i][j]: 順列の最初の i 個の要素を決める方法であって, まだ使っていない整数のうち
    //           p_(i-1) よりも大きい要素が j 個である場合で, 条件を満たすものの個数
    // 数の大小にしか興味はないので, まとめて数えてよい, という考えに思えた
    let mut dp = vec![vec![0; n + 1]; n + 1];
    // 初期化: 長さ 1 の場合はどれを並べても良い
    for i in 0..n {
        dp[1][i] = 1;
    }

    for i in 1..n {
        let mut sumdp = vec![0; n + 2];
        for j in 0..n + 1 {
            sumdp[j + 1] = sumdp[j] + dp[i][j];
        }

        for j in 0..n - i + 1 {
            if s[i - 1] == '<' {
                // 小さい要素が使えない, を -sumdp[j+1] で表現
                dp[i + 1][j] = (sumdp[n - i + 1] - sumdp[j + 1]) % MOD;
            } else {
                // 大きい要素が使えない, ので計算済みの累積和をそのまま利用するだけ
                dp[i + 1][j] = sumdp[j + 1] % MOD;
            }
        }
    }
    println!("{}", dp[n][0]);
}
