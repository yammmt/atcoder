// :fu:

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        wvn: [(usize, usize); n],
    }

    // i 番目品定め後にその期待値を達成する最小の重さを記録
    // dp 対象は期待値に等しくなる最小の重さであり、その期待値以上を達成する最小の重さではない
    // 今回は達成できる最大の期待値で重さ制限以下のものを求めるため問題はない
    let mut dp = vec![vec![std::usize::MAX / 2; 100001]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..dp[i].len() {
            dp[i + 1][j] = if j >= wvn[i].1 {
                // println!("j: {}, wvn[i].1: {}", j, wvn[i].1);
                (dp[i][j - wvn[i].1] + wvn[i].0).min(dp[i][j])
            } else {
                dp[i][j]
            };
        }
        // println!("{:?}", dp);
    }

    let mut ans = 0;
    for i in (0..dp[n].len()).rev() {
        if dp[n][i] <= w {
            ans = i;
            break;
        }
    }
    println!("{}", ans);
}
