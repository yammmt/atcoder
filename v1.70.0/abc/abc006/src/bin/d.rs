use proconio::fastout;
use proconio::input;

const INF: usize = usize::MAX / 2;

#[fastout]
fn main() {
    input! {
        n: usize,
        cn: [usize; n],
    }

    // 最長増加部分列, LIS
    // dp[i]: 長さ i の部分列の最後尾の最小値
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for c in cn {
        let mut l = 0;
        let mut r = n;
        while r - l > 1 {
            let mid = (l + r) / 2;
            // 制約の都合, 等号は発生しない
            if c >= dp[mid] {
                l = mid;
            } else {
                r = mid;
            }
        }
        dp[r] = dp[r].min(c);
        // println!("{:?}", dp);
    }

    for i in (0..=n).rev() {
        if dp[i] != INF {
            println!("{}", n - i);
            return;
        }
    }
}
