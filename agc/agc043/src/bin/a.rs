use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut vdq = VecDeque::new();
    let mut dp = vec![vec![h * w + 1; w]; h];
    dp[0][0] = if s[0][0] == '.' { 0 } else { 1 };
    vdq.push_back((0, 0));
    while !vdq.is_empty() {
        let cur = vdq.pop_front().unwrap();
        if cur.0 + 1 < h {
            let p = dp[cur.0][cur.1];
            if s[cur.0 + 1][cur.1] == '.' {
                // 今の状態によらずコスト追加不要
                if p < dp[cur.0 + 1][cur.1] {
                    dp[cur.0 + 1][cur.1] = p;
                    vdq.push_back((cur.0 + 1, cur.1));
                }
            } else if s[cur.0][cur.1] == '#' && p < dp[cur.0 + 1][cur.1] {
                // まとめて反転するのでコスト追加不要
                dp[cur.0 + 1][cur.1] = p;
                vdq.push_back((cur.0 + 1, cur.1));
            } else if p + 1 < dp[cur.0 + 1][cur.1] {
                // 反転
                dp[cur.0 + 1][cur.1] = p + 1;
                vdq.push_back((cur.0 + 1, cur.1));
            }
        }
        if cur.1 + 1 < w {
            let p = dp[cur.0][cur.1];
            if s[cur.0][cur.1 + 1] == '.' {
                // 今の状態によらずコスト追加不要
                if p < dp[cur.0][cur.1 + 1] {
                    dp[cur.0][cur.1 + 1] = p;
                    vdq.push_back((cur.0, cur.1 + 1));
                }
            } else if s[cur.0][cur.1] == '#' && p < dp[cur.0][cur.1 + 1] {
                // まとめて反転するのでコスト追加不要
                dp[cur.0][cur.1 + 1] = p;
                vdq.push_back((cur.0, cur.1 + 1));
            } else if p + 1 < dp[cur.0][cur.1 + 1] {
                // 反転
                dp[cur.0][cur.1 + 1] = p + 1;
                vdq.push_back((cur.0, cur.1 + 1));
            }
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
