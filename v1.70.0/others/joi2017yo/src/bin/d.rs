// 難しい

use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    const DUMMY_BIG: usize = usize::MAX / 2;

    input! {
        n: usize,
        m: usize,
        an: [Usize1; n],
    }

    // 並び順を決め打ちすれば, 値が異なるところだけを全部引き出して swap して戻せばよい, ので
    // 値が異なるものの数の最小値が答えとなることはわかる.
    // が, 決め打ちする並び順が最大 20! 通りであり, 愚直は通らない.
    // DP によってこの全探索をうまいこと扱う問題, らしい.

    // num[i][x]: 入力数列 A の先頭 x 個に含まれる i の個数
    // これにより, 入力数列の区間 [l, r) において値が i でない箇所の個数は,
    // r-l-(num[i][r]-num[i][l]) で高速に求められるようになる
    let mut num = vec![vec![0; n + 1]; m];
    for i in 0..m {
        for j in 0..n {
            num[i][j + 1] = num[i][j];
            if an[j] == i {
                num[i][j + 1] += 1;
            }
        }
    }

    // dp[S]: 入力数列のうち, 整数値 S で表される集合に含まれる種類の値のみを
    //        左から順に並べて得られる数列と, 入力数列の先頭からその個数の分だけ
    //        取り出して得られる数列とで, 値が異なる箇所数の最小値
    let mut dp = vec![DUMMY_BIG; 1 << m];
    dp[0] = 0;

    for s in 0..(1 << m) {
        // 並べ終わっている分
        let mut left = 0;
        for i in 0..m {
            if (s & (1 << i)) > 0 {
                left += num[i][n];
            }
        }

        for i in 0..m {
            // 配る DP, i を新しく並び替える
            if s & (1 << i) > 0 {
                // 訪問済
                continue;
            }

            // 並べ終わっている分 + これから並べる対象分
            let right = left + num[i][n];
            let added = right - left - (num[i][right] - num[i][left]);

            let ss = s | (1 << i);
            dp[ss] = dp[ss].min(dp[s] + added);
        }
    }
    // println!("{:?}", dp);

    println!("{}", dp.last().unwrap());
}
