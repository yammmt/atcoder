use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    const MOD: usize = 1_000_000_007;
    input! {
        h: usize,
        w: usize,
        shw: [Chars; h],
    }

    // 愚直だと 2000x2000 マスに対する横/縦/斜めに 2000 回の判定が入って TLE
    // 左上から右方向にでも配りながら進めば解けそう
    // が, 本に合わせて貰う DP をする

    let mut dp = vec![vec![0; w]; h];
    // x[r][c]: (r, c) から左方向に進んでいき, 到達可能な範囲までの累積和
    // y[r][c]: (r, c) から上方向に進んでいき, 到達可能な範囲までの累積和
    // z[r][c]: (r, c) から左上方向に進んでいき, 到達可能な範囲までの累積和
    let mut x = vec![vec![0; w + 1]; h + 1];
    let mut y = vec![vec![0; w + 1]; h + 1];
    let mut z = vec![vec![0; w + 1]; h + 1];

    dp[0][0] = 1;
    x[0][1] = 1;
    y[1][0] = 1;
    z[1][1] = 1;

    for r in 0..h {
        for c in 0..w {
            if shw[r][c] == '#' {
                x[r][c + 1] = 0;
                y[r + 1][c] = 0;
                z[r + 1][c + 1] = 0;
                continue;
            }

            dp[r][c] = (dp[r][c] + x[r][c] + y[r][c] + z[r][c]) % MOD;

            x[r][c + 1] = (x[r][c] + dp[r][c]) % MOD;
            y[r + 1][c] = (y[r][c] + dp[r][c]) % MOD;
            z[r + 1][c + 1] = (z[r][c] + dp[r][c]) % MOD;
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
