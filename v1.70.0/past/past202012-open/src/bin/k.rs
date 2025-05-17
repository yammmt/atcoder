use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    const DUMMY: f64 = 1.0e+20;

    input! {
        s4: [Chars; 4],
    }
    let dxy = [(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)];

    // dp[s]: グリッドの状態が s で表される状態から開始して, ボールを投げる必要のある
    //        回数の期待値の最小値
    // マス (i, j) を 4i+j で表し, 的があれば 1

    let mut grid = 0;
    for i in 0..4 {
        for j in 0..4 {
            if s4[i][j] == '#' {
                grid |= 1 << (i * 4 + j);
            }
        }
    }

    let mut dp = [DUMMY; 1 << 16];
    dp[0] = 0.0;

    for s in 0..2usize.pow(16) {
        for i in 0..4 {
            for j in 0..4 {
                let mut s2s = vec![];
                for &(dx, dy) in &dxy {
                    let i2 = (i as usize).wrapping_add_signed(dx);
                    let j2 = (j as usize).wrapping_add_signed(dy);
                    // 範囲外
                    if i2 >= 4 || j2 >= 4 {
                        continue;
                    }

                    // 的のないマス
                    if s & (1 << (i2 * 4 + j2)) == 0 {
                        continue;
                    }

                    // 新たなグリッド
                    let s2 = s & !(1 << (i2 * 4 + j2));
                    s2s.push(s2);
                }

                if s2s.is_empty() {
                    continue;
                }

                let mut sum_dp = 0.0;
                for &x in &s2s {
                    sum_dp += dp[x];
                }
                dp[s] = dp[s].min(sum_dp / s2s.len() as f64 + 5.0 / s2s.len() as f64);
            }
        }
    }

    println!("{}", dp[grid]);
}
