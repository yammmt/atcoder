use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // 貪欲に "(" 発見後 ")" を見つけたら間を全部消す, で無事に O(N^2)
    // "(" の出現位置を stack に積む -> ")" 遭遇時に削除区間をメモる
    // 出力時はいもす法の要領で O(N) で生き残りを回収する

    let mut removed_pos = vec![0; n + 1];
    let mut left_bracket_pos = vec![];
    for (i, &c) in s.iter().enumerate() {
        if c == '(' {
            left_bracket_pos.push(i);
        } else if c == ')' {
            if let Some(l) = left_bracket_pos.pop() {
                removed_pos[l] += 1;
                removed_pos[i + 1] -= 1;
            }
        }
    }

    let mut ans = Vec::with_capacity(n);
    let mut removed_cnt = 0;
    for (i, &c) in s.iter().enumerate() {
        removed_cnt += removed_pos[i];
        if removed_cnt == 0 {
            ans.push(c);
        }
    }

    println!("{}", ans.iter().join(""));
}
