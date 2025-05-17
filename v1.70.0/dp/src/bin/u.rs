use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        ann: [[isize; n]; n],
    }

    // dp[i]: すでになんらかのグループに入れた要素の集合を表す整数値 i に対し, 点数の最大値
    //        or 集合 i に属するうさぎのグループ分けの最大得点

    // i はグループ分けが完了したうさぎの集合であって, うさぎがどこのグループにいるかの
    // 情報はもっていないのでは？とすると, 点数の加算はどうやって？
    // => dp 回す際に同一グループの候補を全部確認する
    // 計算量回析がよくわからぬ

    let bit_max = 1 << n;

    let mut score = vec![0; bit_max];
    for s in 0..bit_max {
        for i in 0..n {
            for j in i + 1..n {
                if s & (1 << i) == 0 || s & (1 << j) == 0 {
                    continue;
                }

                score[s] += ann[i][j];
            }
        }
    }

    let mut dp = vec![0; bit_max];
    for s in 0..bit_max {
        let mut t = s;
        loop {
            dp[s] = dp[s].max(dp[t] + score[s ^ t]);

            if t == 0 {
                // loop 側では弾けない, 判定漏れになるので
                break;
            }
            // &s がなければ部分集合でない場合が混じって WA
            t = (t - 1) & s;
        }
    }

    println!("{}", dp.last().unwrap());
}
