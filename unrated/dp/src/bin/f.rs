// 片方の部分列を全列挙すると 2^(s.len()) で TLE
// DP 探索しながらベクトルを複製していくと TLE/MLE っぽい (表示は RE)

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
    for i in 0..s.len() {
        for j in 0..t.len() {
            dp[i + 1][j + 1] = if s[i] == t[j] {
                dp[i][j] + 1
            } else {
                dp[i][j + 1].max(dp[i + 1][j])
            };
        }
    }
    // println!("{:?}", dp);

    let mut ans = vec![];
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
            j -= 1;
            ans.push(s[i]);
        }
    }
    ans.reverse();
    println!("{}", ans.iter().collect::<String>());
}
