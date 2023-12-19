use proconio::fastout;
use proconio::input;

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        an: [usize; n],
    }

    // 一人の子供に与えられる飴数は [0, an[i]]
    // dp[i][j]: i 人目終了時に j 個の飴を配った分け方の数
    // としたいが j は最大 10^5 となるので単純な遷移は座圧しても TLE
    // 配り方は区間への加算になるのでいもす法でまとめたい
    // ((1, 3), 5) で配布量 [1, 3) の区間には 5 通りの遷移がある, とする？
    // そんなことしなくとも 10^5 通りの配り方を 100 回遷移で計算量 10^7 くらいにおさまるはず

    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for a in an {
        let mut imos = vec![0; k + 1];
        for i in 0..=k {
            imos[i] = (imos[i] + dp[i]) % MOD;

            let section_end = i + a + 1;
            if section_end <= k {
                imos[section_end] = (imos[section_end] + MOD - dp[i]) % MOD;
            }
        }

        let mut dp_nxt = vec![0; k + 1];
        let mut cur = 0;
        for i in 0..=k {
            cur = (cur + MOD + imos[i]) % MOD;
            dp_nxt[i] = cur;
        }

        dp = dp_nxt;
    }

    println!("{}", dp[k]);
}
