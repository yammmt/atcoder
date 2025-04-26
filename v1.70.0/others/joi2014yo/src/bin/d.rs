use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

const MOD: usize = 10007;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // dp[i][j]: bit 集合 i が j 日目に存在する場合の数
    //           bit 集合は下位から順に J, O, I
    let mut dp = vec![vec![0; n + 1]; 8];
    dp[1][0] = 1;

    for (i, c) in s.iter().enumerate() {
        let supervisor_bit = match c {
            'J' => 1,
            'O' => 2,
            'I' => 4,
            _ => unreachable!(),
        };
        for j in 1..8 {
            for k in 1..8 {
                if dp[k][i] == 0 {
                    continue;
                }

                if (j & supervisor_bit > 0) && (j & k > 0) {
                    dp[j][i + 1] = (dp[j][i + 1] + dp[k][i]) % MOD;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 0..8 {
        ans = (ans + dp[i][n]) % MOD;
    }
    println!("{ans}");
}
