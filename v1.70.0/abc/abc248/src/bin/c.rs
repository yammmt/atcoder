use proconio::fastout;
use proconio::input;

// const DUMMY: usize = usize::MAX / 4;
const MOD: usize = 998_244_353;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    // dp[i]: 和が i である数列の総数
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for _ in 0..n {
        let mut cur = vec![0; k + 1];
        for i in 0..k + 1 {
            if dp[i] == 0 {
                continue;
            }

            for j in 1..=m {
                let sum_nxt = i + j;
                if sum_nxt > k {
                    break;
                }

                cur[sum_nxt] = (cur[sum_nxt] + dp[i]) % MOD;
            }
        }
        dp = cur;
    }

    let mut ans = 0;
    for d in &dp {
        ans = (ans + *d) % MOD;
    }

    println!("{ans}");
}
