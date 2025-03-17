use proconio::fastout;
use proconio::input;

const LOOP_COUNT_MAX: usize = 1000;

#[fastout]
fn main() {
    input! {
        n: usize,
        an: [usize; n],
    }

    // 平均値の最大化
    // PAST 本だと判定対象を先に引いて 0 以上か否かで判定しているが, そうせずとも解けるはず
    // …と思ったが, 選択数が不定だから 0 基準で判定しないとだめか
    let mut pass = 0.0;
    let mut fail = 1_000_000_001.0;
    let mut loop_count = 0;
    while fail - pass > 1.0e-5 {
        let mid = (pass + fail) / 2.0;
        let mut bn = Vec::with_capacity(n);
        for &a in &an {
            bn.push(a as f64 - mid);
        }

        // dp[i][j]: i-1 番目の要素を選んだ (j=0)/選ばなかった (j=1) 場合の最大値
        let mut dp = vec![vec![0.0f64; 2]; n + 1];
        for i in 0..n {
            dp[i + 1][0] = dp[i][0].max(dp[i][1]) + bn[i] as f64;
            dp[i + 1][1] = dp[i][0];
        }

        if dp[n][0] >= 0.0 || dp[n][1] >= 0.0 {
            pass = mid;
        } else {
            fail = mid;
        }
        loop_count += 1;
        if loop_count == LOOP_COUNT_MAX {
            break;
        }
    }
    println!("{pass}");

    // 中央値の最大化
    // 判定対象の数を X として, X 以上を +1, 未満を -1 と置けば合計値 >= 0 問題に帰着できる
    let mut pass = 0;
    let mut fail = 1_000_000_001;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let mut bn = Vec::with_capacity(n);
        for &a in &an {
            bn.push(if a >= mid { 1 } else { -1 });
        }

        // dp[i][j]: i-1 番目の要素を選んだ (j=0)/選ばなかった (j=1) 場合の最大値
        let mut dp = vec![vec![0; 2]; n + 1];
        for i in 0..n {
            dp[i + 1][0] = dp[i][0].max(dp[i][1]) + bn[i];
            dp[i + 1][1] = dp[i][0];
        }

        if dp[n][0] > 0 || dp[n][1] > 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{pass}");

    // TODO: 一関数にトレイト制約乗せれば判定共通化できそう
}
