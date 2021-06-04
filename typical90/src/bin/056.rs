// 11.5min DP の経路復元

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        abn: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; s + 1]; n + 1];
    dp[0][0] = 1;
    // 配る DP
    for (i, ab) in abn.iter().enumerate() {
        for j in 0..s {
            if dp[i][j] == 0 {
                continue;
            }

            let next_s = [j + ab.0, j + ab.1];
            for m in &next_s {
                if *m <= s {
                    dp[i + 1][*m] += dp[i][j];
                }
            }
        }
    }
    // println!("{:?}", dp[n]);

    if dp[n][s] == 0 {
        println!("Impossible");
        return;
    }

    let mut ans = vec![];
    let mut cur_s = s;
    for i in (1..n + 1).rev() {
        if cur_s >= abn[i - 1].0 && dp[i - 1][cur_s - abn[i - 1].0] > 0 {
            ans.push('A');
            cur_s -= abn[i - 1].0;
        } else {
            ans.push('B');
            cur_s -= abn[i - 1].1;
        }
        // println!("{:?}", ans);
    }

    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
