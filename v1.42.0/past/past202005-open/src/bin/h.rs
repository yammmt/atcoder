// 短く書きたい

use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        l: usize,
        xn: [usize; n],
        t3: [usize; 3],
    }
    let mut hurdles = HashSet::new();
    for x in &xn {
        hurdles.insert(2 * x);
    }

    let idx_max = 2 * l + 1;
    // dp[0]: ground, dp[1]: jumping
    let mut dp = vec![vec![std::usize::MAX / 2; idx_max]; 2];
    dp[0][0] = 0;
    dp[1][0] = 0;
    for i in 0..2 * l + 1 {
        if i % 2 == 1 {
            // jumping
            continue;
        }

        // walk
        if i + 2 < idx_max {
            let cur = if hurdles.contains(&(i + 2)) {
                dp[0][i] + t3[0] + t3[2]
            } else {
                dp[0][i] + t3[0]
            };
            dp[0][i + 2] = dp[0][i + 2].min(cur);
        }

        // jump 1
        for j in 2..4 {
            if i + j < idx_max {
                dp[1][i + j] = (dp[1][i + j]).min(dp[0][i] + t3[0] / 2 + (j - 1) * t3[1] / 2);
            }
        }
        if i + 4 < idx_max {
            let cur = if hurdles.contains(&(i + 4)) {
                dp[0][i] + t3[0] + t3[1] + t3[2]
            } else {
                dp[0][i] + t3[0] + t3[1]
            };
            dp[0][i + 4] = (dp[0][i + 4]).min(cur);
        }

        // jump 2
        for j in 4..8 {
            if i + j < idx_max {
                dp[1][i + j] = (dp[1][i + j]).min(dp[0][i] + t3[0] / 2 + (j - 1) * t3[1] / 2);
            }
        }
        if i + 8 < idx_max {
            let cur = if hurdles.contains(&(i + 8)) {
                dp[0][i] + t3[0] + 3 * t3[1] + t3[2]
            } else {
                dp[0][i] + t3[0] + 3 * t3[1]
            };
            dp[0][i + 8] = (dp[0][i + 8]).min(cur);
        }
    }

    println!("{}", dp[0][2 * l].min(dp[1][2 * l]));
}
