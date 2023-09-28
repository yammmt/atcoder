use proconio::input;

static DP_LEN: usize = 301;
static DUMMY: isize = std::isize::MAX / 4;

#[allow(clippy::needless_range_loop)]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        abn: [(usize, usize); n],
    }

    // dp[i][j]: i 個のたこ焼きと j 個のたい焼きを手に入れる際の最小費用
    // 300 個を超えたならそれ以上カウントしなくて良い
    let mut dp = vec![vec![DUMMY; DP_LEN]; DP_LEN];
    dp[0][0] = 0;
    // println!("{:?}", dp);
    for ab in &abn {
        let mut cur = dp.clone();
        for i in 0..DP_LEN {
            let next_i = (i + ab.0).min(DP_LEN - 1);
            for j in 0..DP_LEN {
                let next_j = (j + ab.1).min(DP_LEN - 1);
                cur[next_i][next_j] = cur[next_i][next_j].min(dp[i][j] + 1);
            }
        }
        // println!("{:?}", cur);
        dp = cur;
    }

    let mut ans = DUMMY;
    for i in x..DP_LEN {
        for j in y..DP_LEN {
            ans = ans.min(dp[i][j]);
        }
    }
    println!("{}", if ans == DUMMY { -1 } else { ans });
}
