// :fu: 21-03 EDPC のゲーム系見てから

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        ss: String,
        x: Chars,
    }

    let sss = ss.chars().collect::<Vec<char>>();
    let mut s = vec![];
    for c in &sss {
        s.push(c.to_digit(10).unwrap() as usize);
    }

    // takahashi win -> true
    let mut dp = vec![vec![false; 7]; n + 1];
    dp[n][0] = true;
    let mut ten = 1;
    for i in (0..n).rev() {
        if x[i] == 'T' {
            for j in 0..7 {
                dp[i][j] = dp[i + 1][j] || dp[i + 1][(j + ten * s[i]) % 7];
            }
        } else {
            for j in 0..7 {
                dp[i][j] = dp[i + 1][j] && dp[i + 1][(j + ten * s[i]) % 7];
            }
        };
        ten = (ten * 10) % 7;
    }

    println!(
        "{}",
        if dp[0][0] {
            "Takahashi"
        } else {
            "Aoki"
        }
    );
}
