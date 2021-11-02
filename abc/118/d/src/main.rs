// :fu: 21-11 解きたい

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        am: [usize; m],
    }

    let matches = [2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut can_use = [false; 9];
    am.iter().for_each(|&a| can_use[a - 1] = true);

    // dp[i]: i 個のマッチを使ってできる数の最大桁数
    // 初期値を 0 にしてしまうと存在しない位置から探索されてしまう
    let mut dp = vec![-1_000_000_000; n + 1];
    dp[0] = 1;
    for i in 0..n {
        for j in 0..9 {
            if !can_use[j] {
                continue;
            }

            let next_i = i + matches[j];
            if next_i > n {
                continue;
            }

            dp[next_i] = dp[next_i].max(dp[i] + 1);
        }
    }

    // 経路復元
    let mut ans = vec![];
    let mut i = n;
    while i > 0 {
        for j in (0..9).rev() {
            if can_use[j] && matches[j] <= i && dp[i - matches[j]] == dp[i] - 1 {
                // println!("  => {}", j + 1);
                ans.push(j + 1);
                i -= matches[j];
                break;
            }
        }
    }

    for a in &ans {
        print!("{}", a);
    }
    println!();
}
