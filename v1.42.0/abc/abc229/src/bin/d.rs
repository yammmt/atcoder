// :fu: 21-11 しゃくとり法はいつもバグる
// WA: 問題文を読む

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        k: usize,
    }

    // 最低 k 個は連続させられる
    // 連続の始点を固定してしまえば後は繋げられるだけ繋げ続ける

    let mut ans = 0;
    let mut replaced_dot = 0;
    let mut right_i = 0;

    for left_i in 0..s.len() {
        while right_i < s.len() {
            if s[right_i] == '.' && replaced_dot < k {
                replaced_dot += 1;
            } else if s[right_i] == '.' && replaced_dot == k {
                break;
            }
            right_i += 1;
        }
        ans = ans.max(right_i - left_i);

        if right_i == left_i {
            right_i += 1;
        } else if s[left_i] == '.' {
            replaced_dot -= 1;
        }
    }

    println!("{}", ans);
}
