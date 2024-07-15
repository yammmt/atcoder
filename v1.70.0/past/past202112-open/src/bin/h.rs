// FIXME: テンプレ解法でない考え方で求まらない理由がわからん
// https://atcoder.jp/contests/past202112-open/submissions/55635279
// s を基準に, dp[i] を非共通部分列の長さが i である場合の t の最後の要素の位置と定義する。
// dp[] を十分大きなダミー値として初期化する。
// s の各要素に対して DP を回し、非ダミー値が入っている最大のインデックスが答え

// ↑は↓の提出と同方針？
//     https://atcoder.jp/contests/past202112-open/submissions/29112111

use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    // dp[i][j]: s[i], t[j] まで見ての最長非共通部分列の長さ
    let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

    for i in 0..=s.len() {
        for j in 0..=t.len() {
            if i != s.len() {
                // 今見ているものを採用しない
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            }
            if j != t.len() {
                // 今見ているものを採用しない
                dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
            }
            if i != s.len() && j != t.len() && s[i] != t[j] {
                // 今見ているものを採用する
                dp[i + 1][j + 1] = dp[i + 1][j + 1].max(dp[i][j] + 1);
            }
        }
    }

    println!("{}", dp[s.len()][t.len()]);
}
