use proconio::input;

static MOD_OF: usize = 998_244_353;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = vec![1; 10];
    dp[0] = 0;
    for _ in 0..n - 1 {
        let mut cur = vec![0; 10];
        for i in 1..=9 {
            cur[i] = (cur[i] + dp[i]) % MOD_OF;

            if i < 9 {
                cur[i + 1] = (cur[i + 1] + dp[i]) % MOD_OF;
            }

            if i > 1 {
                cur[i - 1] = (cur[i - 1] + dp[i]) % MOD_OF;
            }
        }
        dp = cur;
    }

    let mut ans = 0;
    dp.iter().for_each(|&d| ans = (ans + d) % MOD_OF);

    println!("{ans}");
}
