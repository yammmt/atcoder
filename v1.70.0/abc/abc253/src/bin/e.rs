// TODO: EDPC M が類題らしい

use proconio::fastout;
use proconio::input;

const MOD: usize = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
    }

    // dp[i][j]: i 番目の値が決まった時点で最後の数列が j である数の個数
    // としたいが 25,000,000,000 回くらいのループとなり TLE
    // 絶対値に制約があると更新先の個数はいもす法で扱えそう
    // dp[ij[j] それぞれに対し j 回代入した分を区間操作に入れ替えると
    // dp[i+1][jj] を計算するコストが O(M) に落ちそうで, すると O(N*(M+M)) っぽい
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..=m {
        // 初手はなんでもよい
        dp[1][i] = 1;
    }

    for i in 2..=n {
        let mut imos = vec![0; m + 1];
        for j in 1..=m {
            if k == 0 {
                // a[i] == a[i+1] のときは k は必ず 0
                // k が 0 ならなにをしてもよい
                imos[1] = (imos[1] + dp[i - 1][j]) % MOD;
            } else {
                // 等号時の処理がなぁと思ったが, a[i]-a[i+1]==k と a[i+1]-a[i]==k とで
                // k==0 時を別処理に分けてさえいればこれらの値は衝突しない はず

                // a[i] > a[i+1] のとき, 1 <= a[i+1] < a[i]-k
                let ai_minus_k = j as i64 - k;
                if ai_minus_k >= 1 {
                    let ai_minus_k = ai_minus_k as usize;
                    imos[1] = (imos[1] + dp[i - 1][j]) % MOD;
                    // a[i+1] == a[i]-k のときは成り立つので引かない
                    imos[ai_minus_k + 1] = (imos[ai_minus_k + 1] + MOD - dp[i - 1][j]) % MOD;
                }

                // a[i] < a[i+1] のとき, a[i]+k < a[i+1] <= m
                let ai_plus_k = j as i64 + k;
                if ai_plus_k <= m as i64 {
                    let ai_plus_k = ai_plus_k as usize;
                    imos[ai_plus_k] = (imos[ai_plus_k] + dp[i - 1][j]) % MOD;
                    // 減算位置は m+1 となるので不要
                }
            }
        }

        let mut cur = 0;
        for j in 1..=m {
            cur = (cur + imos[j]) % MOD;
            dp[i][j] = cur;
        }
    }

    let mut ans = 0;
    for d in &dp[n] {
        ans = (ans + *d) % MOD;
    }

    println!("{ans}");
}
