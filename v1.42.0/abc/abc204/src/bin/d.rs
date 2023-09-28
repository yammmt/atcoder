// :fu: :fu: 21-06 緑前半は嘘寄り
// WA: 貪欲

use proconio::input;

fn main() {
    input! {
        n: usize,
        tn: [usize; n],
    }

    // DP で片方を全通り列挙してもう片方と比較しても最悪 2^N 通り列挙することになり TLE?
    let tsum = tn.iter().sum::<usize>();
    let mut dp = vec![vec![false; tsum + 1]; n + 1];
    dp[0][0] = true;
    for (i, t) in tn.iter().enumerate() {
        for j in 0..tsum + 1 {
            if !dp[i][j] {
                continue;
            }

            dp[i + 1][j] = true;
            let next_j = j + *t;
            if next_j > tsum {
                continue;
            }

            dp[i + 1][next_j] = true;
        }
    }
    // println!("{:?}", dp);

    for i in (tsum + 1) / 2..tsum + 1 {
        if dp[n][i] {
            println!("{}", i);
            return;
        }
    }
}
