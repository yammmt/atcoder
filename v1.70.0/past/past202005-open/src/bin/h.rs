use proconio::fastout;
use proconio::input;
use std::collections::HashSet;

static DUMMY: usize = std::usize::MAX / 4;

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        xn: [usize; n],
        t3: [usize; 3],
    }

    let hurdles: HashSet<usize> = HashSet::from_iter(xn.iter().map(|&x| 2 * x));

    // 0.5 がだるいので距離を 2 倍にして配る DP
    // dp[i]: i 到着時の最短, 通過時でないのでハードルがこの地点にあるなら加算されていない
    // 場合分け辛くなるしメモリ容量にも余裕あるのでサイズにマージンとっておく
    let mut dp = vec![DUMMY; 2 * l + 1 + 10];
    dp[0] = 0;
    let mut i = 0;
    while i + 1 < 2 * l + 1 {
        let hurdle_cur = if hurdles.contains(&i) { t3[2] } else { 0 };

        // 行動 1
        dp[i + 1] = dp[i + 1].min(dp[i] + t3[0] / 2 + hurdle_cur);
        dp[i + 2] = dp[i + 2].min(dp[i] + t3[0] + hurdle_cur);

        // 行動 2/3 では空中にいながらゴールする可能性がある
        // 通常時の DP 遷移で空中を考慮すると空中で走り始めてしまうので,
        // ゴール到達時にのみ到着判定を入れるようにする

        // 行動 2
        // 着時時にはハードルはない (元の座標系で位置 0.5)
        if i + 2 == 2 * l {
            dp[i + 2] = dp[i + 2].min(dp[i] + t3[0] / 2 + t3[1] / 2 + hurdle_cur);
        }
        dp[i + 4] = dp[i + 4].min(dp[i] + t3[0] + t3[1] + hurdle_cur);

        // 行動 3
        if i + 4 == 2 * l {
            dp[i + 4] = dp[i + 4].min(dp[i] + t3[0] / 2 + t3[1] + t3[1] / 2 + hurdle_cur);
        }
        if i + 6 == 2 * l {
            dp[i + 6] = dp[i + 6].min(dp[i] + t3[0] / 2 + 2 * t3[1] + t3[1] / 2 + hurdle_cur);
        }
        dp[i + 8] = dp[i + 8].min(dp[i] + t3[0] + 3 * t3[1] + hurdle_cur);

        i += 2;
    }

    println!("{}", dp[2 * l]);
}
