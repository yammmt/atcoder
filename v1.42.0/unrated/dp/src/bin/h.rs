use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        ahw: [Chars; h],
    }
    let d = 10u64.pow(9) + 7;

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    let dir = vec![(1, 0), (0, 1)];
    for i in 0..h {
        for j in 0..w {
            // println!("{:?}", dp);
            if ahw[i][j] == '#' {
                continue;
            }
            // println!("({}, {})", i, j);

            for dr in &dir {
                // println!("  dr: {:?}", dr);
                let next_h = i + dr.0;
                let next_w = j + dr.1;
                if next_h < h && next_w < w && ahw[next_h][next_w] != '#' {
                    // println!("  update ({}, {})", next_h, next_w);
                    dp[next_h][next_w] = (dp[next_h][next_w] + dp[i][j]) % d;
                }
            }
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp[h - 1][w - 1]);
}
