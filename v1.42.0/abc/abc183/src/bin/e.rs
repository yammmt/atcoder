use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let d = 10u64.pow(9) + 7;

    // 累積和を事前に計算するとどうしても合わないのなぜかわからない
    let mut dp = vec![vec![0; w]; h];
    let mut dph = vec![vec![0; w]; h];
    let mut dpw = vec![vec![0; w]; h];
    let mut dphw = vec![vec![0; w]; h];
    dp[0][0] = 1;
    dph[0][0] = 1;
    dpw[0][0] = 1;
    dphw[0][0] = 1;
    // 貰う DP
    for i in 0..h {
        for j in 0..w {
            // println!("{}, {}", i, j);
            if s[i][j] == '#' || (i == 0 && j == 0) {
                continue;
            }

            if i > 0 {
                dph[i][j] = (dph[i][j] + dph[i - 1][j]) % d;
            }
            if j > 0 {
                dpw[i][j] = (dpw[i][j] + dpw[i][j - 1]) % d;
            }
            if i > 0 && j > 0 {
                dphw[i][j] = (dphw[i][j] + dphw[i - 1][j - 1]) % d;
            }

            dp[i][j] = (dph[i][j] + dpw[i][j] + dphw[i][j]) % d;

            dph[i][j] = (dph[i][j] + dp[i][j]) % d;
            dpw[i][j] = (dpw[i][j] + dp[i][j]) % d;
            dphw[i][j] = (dphw[i][j] + dp[i][j]) % d;
            // println!("{:?}", dp);
        }
    }

    println!("{}", dp[h - 1][w - 1]);
}
