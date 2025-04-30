use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    const MOD: usize = 1_234_567;
    input! {
        n: usize,
        p: usize,
        hn: [usize; n],
    }

    // 制約に h_k <= p がない, 解答 0 も発生しうる
    // 愚直に O(N^2) で解いても部分点が取れる
    // いもす法とイベントソートとで解ける気がするが, 本に合わせる
    // 累積の高さを出して二分探索でも通りそう

    // 各 i に対し, 数列 H の区間 [x, i) の総和が P 以下になる最小の x[i] を尺取法で求める
    let mut x = vec![0; n + 1];
    let mut left = 0;
    let mut sum_interval = 0;
    for right in 0..=n {
        while left < right && sum_interval > p {
            sum_interval -= hn[left];
            left += 1;
        }

        x[right] = left;

        if right < n {
            sum_interval += hn[right];
        }
    }

    // dp[i]: 区間 [0, i) に対する分割の仕方の個数
    let mut dp = vec![0; n + 1];
    dp[0] = 1;
    // sum_dp[i]: 配列 dp の累積和で, sum_dp[i] は dp[0:i] の和
    let mut sum_dp = vec![0; n + 2];
    sum_dp[1] = 1;

    for i in 1..=n {
        dp[i] = (sum_dp[i] + MOD - sum_dp[x[i]]) % MOD;
        sum_dp[i + 1] = (sum_dp[i] + dp[i]) % MOD;
    }
    // println!("{:?}", dp);
    // println!("{:?}", sum_dp);

    println!("{}", dp[n]);
}
