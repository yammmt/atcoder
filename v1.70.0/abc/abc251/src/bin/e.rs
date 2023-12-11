use proconio::fastout;
use proconio::input;

const DUMMY: usize = usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 焼きなますと通ったりして

    // dp[i][j][k]: i 番目まで判断を終えて j=1 なら直前に餌をあげた, 0 ならあげなかった
    // k=0 なら 1 は餌をもらっていない
    // 直前にあげていなければ今回あげなければならない
    // 最後のために 1 にあげた/あげなかったは別管理にする
    let mut dp = vec![vec![vec![DUMMY; 2]; 2]; n + 1];
    dp[0][0][0] = 0;
    dp[0][0][1] = 0;
    dp[0][1][0] = 0;
    dp[0][1][1] = 0;
    for i in 1..=n {
        if i == n {
            // 直前に餌をもらっており, かつ初手で餌を与えた場合を除き餌が必須
            // dp の添字わかんなくなるのでもうここで回答する
            println!(
                "{}",
                (dp[n - 1][0][0] + an[n - 1]).min(
                    (dp[n - 1][1][0] + an[n - 1])
                        .min((dp[n - 1][0][1] + an[n - 1]).min(dp[n - 1][1][1]))
                )
            );
        } else {
            // 直前にもらっていないと遷移不可
            dp[i][0][0] = dp[i - 1][1][0];
            dp[i][0][1] = dp[i - 1][1][1];
            // 今回餌をあげるなら直前の行動は不問
            dp[i][1][0] = (dp[i - 1][0][0] + an[i - 1]).min(dp[i - 1][1][0] + an[i - 1]);
            dp[i][1][1] = (dp[i - 1][0][1] + an[i - 1]).min(dp[i - 1][1][1] + an[i - 1]);
        }
    }
}
